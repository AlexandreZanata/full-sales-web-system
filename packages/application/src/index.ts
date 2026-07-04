export type { ApplicationErrorBody } from './errors.js';
export { ApplicationError } from './errors.js';
export { applicationDomainRef } from './scaffold.js';
export type { CommerceRepository, ProductRepository, SaleRepository } from './ports/index.js';
export { CreateSaleHandler, GetSaleHandler, ListProductsHandler } from './handlers/sales.js';
export type {
  CreateSaleInput,
  CreateSaleOutput,
  GetSaleOutput,
  ListProductsOutput,
} from './handlers/sales.js';
export {
  InMemoryCommerceRepository,
  InMemoryProductRepository,
  InMemorySaleRepository,
} from './adapters/in-memory.js';
