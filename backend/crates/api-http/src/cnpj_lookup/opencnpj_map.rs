use chrono::Utc;
use serde::Deserialize;
use serde_json::Value;

use super::{CnpjLookupAddress, CnpjLookupCnae, CnpjLookupPartner, CnpjLookupResult};

#[derive(Deserialize)]
struct OpenCnpjEndereco {
    logradouro: Option<String>,
    numero: Option<String>,
    complemento: Option<String>,
    bairro: Option<String>,
    municipio: Option<String>,
    uf: Option<String>,
    cep: Option<String>,
}

#[derive(Deserialize)]
struct OpenCnpjCnae {
    codigo: Option<String>,
    descricao: Option<String>,
}

#[derive(Deserialize)]
struct OpenCnpjSocio {
    nome: String,
    qualificacao: Option<String>,
    data_entrada_sociedade: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct PublicCnpjResponse {
    cnpj: String,
    razao_social: String,
    nome_fantasia: Option<String>,
    situacao_cadastral: Option<String>,
    uf: Option<String>,
    municipio: Option<String>,
    cnae_principal: Option<OpenCnpjCnae>,
    endereco: Option<OpenCnpjEndereco>,
    telefone: Option<String>,
    email: Option<String>,
    socios: Option<Vec<OpenCnpjSocio>>,
}

pub fn map_opencnpj_response(
    body: PublicCnpjResponse,
    upstream_snapshot: Value,
) -> CnpjLookupResult {
    let legal_name = body.razao_social;
    let trade_name = body
        .nome_fantasia
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| legal_name.clone());
    let endereco = body.endereco;
    let city = endereco
        .as_ref()
        .and_then(|addr| addr.municipio.clone())
        .or(body.municipio)
        .unwrap_or_default();
    let state = endereco
        .as_ref()
        .and_then(|addr| addr.uf.clone())
        .or(body.uf)
        .unwrap_or_default();
    let postal_code = endereco
        .as_ref()
        .and_then(|addr| addr.cep.clone())
        .unwrap_or_default()
        .replace(['-', '.'], "");
    let complement = endereco
        .as_ref()
        .and_then(|addr| addr.complemento.clone())
        .filter(|value| !value.trim().is_empty());
    let main_cnae = body.cnae_principal.and_then(|cnae| {
        let code = cnae.codigo.filter(|v| !v.trim().is_empty())?;
        Some(CnpjLookupCnae {
            code,
            description: cnae.descricao.unwrap_or_default(),
        })
    });
    let partners = body.socios.map(|rows| {
        rows.into_iter()
            .map(|socio| CnpjLookupPartner {
                name: socio.nome,
                role: socio.qualificacao,
                joined_at: socio.data_entrada_sociedade,
            })
            .collect()
    });
    CnpjLookupResult {
        cnpj: body.cnpj,
        legal_name: legal_name.clone(),
        trade_name,
        address: CnpjLookupAddress {
            street: endereco
                .as_ref()
                .and_then(|addr| addr.logradouro.clone())
                .unwrap_or_default(),
            number: endereco
                .as_ref()
                .and_then(|addr| addr.numero.clone())
                .unwrap_or_else(|| "S/N".into()),
            complement,
            district: endereco
                .as_ref()
                .and_then(|addr| addr.bairro.clone())
                .unwrap_or_default(),
            city,
            state,
            postal_code,
        },
        phone: body.telefone.filter(|value| !value.trim().is_empty()),
        email: body.email.filter(|value| !value.trim().is_empty()),
        registration_status: body.situacao_cadastral,
        main_cnae,
        partners,
        upstream_snapshot: Some(upstream_snapshot),
        provider: "opencnpj".into(),
        fetched_at: Utc::now(),
    }
}

#[cfg(test)]
mod tests {
    use super::{PublicCnpjResponse, map_opencnpj_response};
    use serde_json::json;

    #[test]
    fn given_del_moro_payload_when_map_then_all_extended_fields() {
        let snapshot = json!({
            "cnpj": "00877761000126",
            "razao_social": "DEL MORO & DEL MORO LTDA",
            "nome_fantasia": "DEL MORO SUPERMERCADOS",
            "situacao_cadastral": "02",
            "telefone": "(66) 35127000",
            "email": "ESCRITORIO@DELMORO.COM.BR",
            "cnae_principal": { "codigo": "4711302", "descricao": "Supermercados" },
            "endereco": {
                "logradouro": "AVENIDA LUDOVICO DA RIVA NETO",
                "numero": "2920",
                "complemento": "SETOR: D;",
                "bairro": "CENTRO",
                "cep": "78580000",
                "uf": "MT",
                "municipio": "ALTA FLORESTA"
            },
            "socios": [{ "nome": "ANTONIO DEL MORO", "qualificacao": "Administrador" }]
        });
        let body: PublicCnpjResponse = serde_json::from_value(snapshot.clone()).expect("body");
        let mapped = map_opencnpj_response(body, snapshot);
        assert_eq!(mapped.cnpj, "00877761000126");
        assert_eq!(mapped.address.complement.as_deref(), Some("SETOR: D;"));
        assert_eq!(mapped.phone.as_deref(), Some("(66) 35127000"));
        assert_eq!(mapped.email.as_deref(), Some("ESCRITORIO@DELMORO.COM.BR"));
        assert_eq!(mapped.registration_status.as_deref(), Some("02"));
        assert_eq!(
            mapped.main_cnae.as_ref().map(|c| c.code.as_str()),
            Some("4711302")
        );
        assert_eq!(mapped.partners.as_ref().map(|p| p.len()), Some(1));
        assert!(mapped.upstream_snapshot.is_some());
    }
}
