type DotPrefix<T extends string, U extends string> = T extends '' ? U : `${T}.${U}`;

type DotPaths<T, Prev extends string = ''> = {
  [K in keyof T & string]: T[K] extends Record<string, unknown>
    ? DotPaths<T[K], DotPrefix<Prev, K>>
    : DotPrefix<Prev, K>;
}[keyof T & string];

export type Messages = {
  nav: {
    dashboard: string;
    users: string;
    commerces: string;
    products: string;
    inventory: string;
    orders: string;
    deliveries: string;
    sales: string;
    reports: string;
    audit: string;
  };
  auth: {
    signIn: string;
    signingIn: string;
    signInTitle: string;
    signInDescription: string;
    email: string;
    password: string;
    logout: string;
    devEnter: string;
    adminLabel: string;
  };
  shell: {
    menu: string;
    openNav: string;
    closeNav: string;
    closeMenu: string;
    locale: string;
    adminNav: string;
    navMenu: string;
  };
  common: {
    previous: string;
    next: string;
    cancel: string;
    confirm: string;
    working: string;
    save: string;
    edit: string;
    remove: string;
    deactivate: string;
    viewAll: string;
    tryAgain: string;
    backToDashboard: string;
    somethingWentWrong: string;
    unexpectedError: string;
    pageNotFound: string;
    jsonPayload: string;
    filter: {
      allStatuses: string;
      allRoles: string;
      allCommerces: string;
      allDrivers: string;
      activeOnly: string;
      inactiveOnly: string;
    };
    active: {
      active: string;
      inactive: string;
    };
    table: {
      date: string;
      status: string;
      total: string;
      name: string;
      email: string;
      role: string;
      product: string;
      qty: string;
      unitPrice: string;
      lineTotal: string;
      paginationAria: string;
    };
    pagination: {
      summary: string;
    };
    loading: {
      default: string;
      saving: string;
      creating: string;
      uploading: string;
      checking: string;
      rejecting: string;
      assigning: string;
      generating: string;
    };
    backTo: {
      users: string;
      commerces: string;
      products: string;
      inventory: string;
      orders: string;
      deliveries: string;
      sales: string;
      reports: string;
    };
  };
  forms: {
    fields: {
      name: string;
      email: string;
      password: string;
      role: string;
      status: string;
      cnpj: string;
      legalName: string;
      tradeName: string;
      street: string;
      number: string;
      district: string;
      city: string;
      state: string;
      postalCode: string;
      phone: string;
      sku: string;
      category: string;
      unitOfMeasure: string;
      price: string;
      currency: string;
      quantity: string;
      reason: string;
      product: string;
      commerce: string;
      driver: string;
      paymentMethod: string;
      declaredPayment: string;
      addressType: string;
      reportType: string;
      periodStart: string;
      periodEnd: string;
      notes: string;
      rejectionReason: string;
      operatingRegion: string;
      monthlyTarget: string;
      cnhNumber: string;
      cnhCategory: string;
      cnhPhoto: string;
      vehiclePlate: string;
      vehicleModel: string;
      vehicleCapacity: string;
      from: string;
      to: string;
      order: string;
      delivery: string;
      sale: string;
      type: string;
      period: string;
      generated: string;
      publicKey: string;
      signature: string;
      verifyResult: string;
      actor: string;
      action: string;
      resource: string;
      resourceId: string;
      details: string;
      time: string;
      commerceId: string;
    };
    placeholders: {
      selectRole: string;
      selectCommerce: string;
      selectProduct: string;
      selectDriver: string;
      selectPaymentMethod: string;
      selectReportType: string;
      selectAddressType: string;
      price: string;
    };
    sections: {
      address: string;
      contact: string;
      lineItems: string;
    };
    validation: {
      nameRequired: string;
      emailRequired: string;
      emailInvalid: string;
      passwordRequired: string;
      passwordMinLength: string;
      roleRequired: string;
      commerceRequired: string;
      commerceContactRequired: string;
      cnpjRequired: string;
      cnpjInvalid: string;
      legalNameRequired: string;
      streetRequired: string;
      numberRequired: string;
      cityRequired: string;
      stateRequired: string;
      postalCodeRequired: string;
      skuRequired: string;
      unitOfMeasureRequired: string;
      priceInvalid: string;
      selectProduct: string;
      selectCommerce: string;
      selectPaymentMethod: string;
      selectDriver: string;
      selectReportType: string;
      periodStartRequired: string;
      periodEndRequired: string;
      periodEndBeforeStart: string;
      reasonRequired: string;
      quantityNonZero: string;
      itemsRequired: string;
      addressTypeRequired: string;
      stateInvalid: string;
      quantityAdjustment: string;
      rejectionReasonRequired: string;
    };
  };
  errors: {
    actionFailed: string;
    uploadFailed: string;
    orders: {
      insufficientStock: string;
      invalidTransition: string;
      rejectionReasonRequired: string;
      deliveryExists: string;
    };
    sales: {
      insufficientStock: string;
      invalidTransition: string;
      inactiveCommerce: string;
      inactiveProduct: string;
      productNotFound: string;
      commerceNotFound: string;
    };
    reports: {
      signingKeyUnavailable: string;
      validationError: string;
      generateFailed: string;
    };
  };
  status: {
    order: {
      Draft: string;
      PendingApproval: string;
      Approved: string;
      Rejected: string;
      Picking: string;
      InTransit: string;
      Delivered: string;
      PartiallyDelivered: string;
      Cancelled: string;
    };
    sale: {
      Pending: string;
      Confirmed: string;
      Cancelled: string;
    };
    delivery: {
      Waiting: string;
      InTransit: string;
      Delivered: string;
      Failed: string;
    };
    report: {
      DailyDriver: string;
      CommercePeriod: string;
      Consolidated: string;
    };
  };
  role: {
    Admin: string;
    Driver: string;
    Seller: string;
    CommerceContact: string;
  };
  payment: {
    cash: string;
    pix: string;
    credit: string;
    debit: string;
    notDeclared: string;
    declaredReceived: string;
    declaredPending: string;
  };
  addressType: {
    Billing: string;
    Delivery: string;
  };
  dashboard: {
    title: string;
    description: string;
    stats: {
      pendingApproval: string;
      deliveriesWaiting: string;
      salesToday: string;
    };
    recentSales: {
      title: string;
      viewAll: string;
      caption: string;
      empty: {
        title: string;
        description: string;
      };
    };
  };
  users: {
    list: {
      title: string;
      description: string;
      newUser: string;
      filterByRole: string;
      caption: string;
      empty: {
        title: string;
        descriptionFiltered: string;
        descriptionDefault: string;
      };
    };
    create: {
      title: string;
      description: string;
      submit: string;
      submitting: string;
    };
    detail: {
      notFound: string;
      tabs: {
        overview: string;
        driverProfile: string;
        sellerProfile: string;
      };
      deactivate: string;
      deactivateDialog: {
        title: string;
        message: string;
        confirm: string;
      };
    };
    driverProfile: {
      save: string;
      saving: string;
    };
    sellerProfile: {
      save: string;
      saving: string;
    };
    toast: {
      deactivated: string;
      driverProfileSaved: string;
      sellerProfileSaved: string;
    };
  };
  commerces: {
    list: {
      title: string;
      description: string;
      register: string;
      filterByStatus: string;
      caption: string;
      empty: {
        title: string;
        descriptionFiltered: string;
        descriptionDefault: string;
      };
    };
    create: {
      title: string;
      description: string;
      submit: string;
      submitting: string;
    };
    detail: {
      notFound: string;
      tabs: {
        overview: string;
        addresses: string;
      };
      deactivate: string;
      deactivateDialog: {
        title: string;
        message: string;
        confirm: string;
      };
    };
    addresses: {
      title: string;
      description: string;
      add: string;
      edit: string;
      save: string;
      saving: string;
      empty: string;
      primaryBadge: string;
      primaryHint: string;
      primaryCheckbox: string;
      primaryConstraint: string;
    };
    logo: {
      title: string;
      label: string;
      saving: string;
    };
    toast: {
      deactivated: string;
      addressAdded: string;
      addressUpdated: string;
      logoUpdated: string;
    };
  };
  products: {
    list: {
      title: string;
      description: string;
      newProduct: string;
      caption: string;
      filterByStatus: string;
      searchPlaceholder: string;
      empty: {
        title: string;
        description: string;
        descriptionFiltered: string;
      };
    };
    create: {
      title: string;
      description: string;
      submit: string;
      submitting: string;
    };
    detail: {
      notFound: string;
      inactiveHint: string;
      deactivate: string;
      deactivateDialog: {
        title: string;
        message: string;
        confirm: string;
      };
    };
    actions: {
      reactivate: string;
    };
    form: {
      save: string;
      saving: string;
      skuReadOnly: string;
    };
    images: {
      title: string;
      uploadHint: string;
      setPrimary: string;
      primary: string;
      secondary: string;
      remove: string;
      removeConfirm: string;
      label: string;
      setPrimaryUpload: string;
      attaching: string;
      loadError: string;
      empty: string;
    };
    stock: {
      available: string;
      units: string;
      loadError: string;
    };
    toast: {
      deactivated: string;
      reactivated: string;
      saved: string;
      updated: string;
      imageAttached: string;
      imageRemoved: string;
      primaryImageUpdated: string;
    };
  };
  inventory: {
    hub: {
      title: string;
      description: string;
      adjustments: {
        title: string;
        description: string;
      };
      ledger: {
        title: string;
        description: string;
      };
    };
    adjustments: {
      title: string;
      description: string;
      quantityHint: string;
      submit: string;
      submitting: string;
    };
    ledger: {
      title: string;
      description: string;
      productFilter: string;
      caption: string;
      selectProduct: {
        title: string;
        description: string;
      };
      empty: {
        title: string;
        description: string;
      };
      columns: {
        type: string;
        quantity: string;
        reason: string;
      };
    };
    toast: {
      adjustmentRecorded: string;
    };
  };
  orders: {
    list: {
      title: string;
      description: string;
      caption: string;
      empty: {
        title: string;
        descriptionFiltered: string;
        descriptionDefault: string;
      };
    };
    detail: {
      notFound: string;
      deliverySection: string;
      lineItems: string;
      actions: {
        approve: string;
        reject: string;
        startPicking: string;
        assignDelivery: string;
        cancel: string;
      };
    };
    rejectDialog: {
      title: string;
      description: string;
      submit: string;
      submitting: string;
    };
    assignDialog: {
      title: string;
      description: string;
      submit: string;
      submitting: string;
    };
    cancelDialog: {
      title: string;
      message: string;
      confirm: string;
    };
    toast: {
      approved: string;
      rejected: string;
      cancelled: string;
      deliveryAssigned: string;
    };
  };
  deliveries: {
    list: {
      title: string;
      description: string;
      filterByStatus: string;
      caption: string;
      empty: {
        title: string;
        description: string;
      };
    };
    detail: {
      notFound: string;
      description: string;
    };
    timeline: {
      title: string;
      failedMessage: string;
    };
  };
  sales: {
    list: {
      title: string;
      description: string;
      newSale: string;
      caption: string;
      empty: {
        title: string;
        descriptionFiltered: string;
        descriptionDefault: string;
      };
    };
    create: {
      title: string;
      description: string;
      addLine: string;
      removeLine: string;
      apiNote: string;
      submit: string;
      submitting: string;
    };
    detail: {
      notFound: string;
      lineItems: string;
      actions: {
        confirm: string;
        cancel: string;
      };
    };
    cancelDialog: {
      title: string;
      message: string;
      confirm: string;
    };
    toast: {
      created: string;
      confirmed: string;
      cancelled: string;
    };
  };
  reports: {
    list: {
      title: string;
      description: string;
      generate: string;
      caption: string;
      empty: {
        title: string;
        description: string;
      };
    };
    generate: {
      title: string;
      description: string;
      commerceOptional: string;
      allCommerces: string;
      adrHint: string;
      submit: string;
      submitting: string;
    };
    detail: {
      notFound: string;
      copyVerifyUrl: string;
      openVerifyEndpoint: string;
      validSignature: string;
      invalidSignature: string;
      checking: string;
      canonicalPayload: string;
    };
    toast: {
      generated: string;
      verifyUrlCopied: string;
    };
  };
  audit: {
    title: string;
    description: string;
    table: {
      caption: string;
    };
    metadata: {
      show: string;
      hide: string;
    };
    empty: {
      title: string;
      description: string;
    };
  };
  uploads: {
    uploading: string;
    noImage: string;
    noFile: string;
    uploadFile: string;
    fileUploaded: string;
    uploadFailed: string;
    previewError: string;
    previewUnavailable: string;
    fileId: string;
  };
};

export type MessageKey = DotPaths<Messages>;
