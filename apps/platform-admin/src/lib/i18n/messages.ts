export type Messages = {
  nav: {
    dashboard: string;
    tenants: string;
    users: string;
    billing: string;
    fraud: string;
    domains: string;
    health: string;
    maintenance: string;
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
    platformLabel: string;
    mfaTitle: string;
    mfaDescription: string;
    mfaCode: string;
    verifyMfa: string;
  };
  shell: {
    openNav: string;
    closeNav: string;
    locale: string;
    navMenu: string;
    impersonating: string;
    endImpersonation: string;
  };
  common: {
    previous: string;
    next: string;
    cancel: string;
    confirm: string;
    save: string;
    search: string;
    viewAll: string;
    tryAgain: string;
    backToDashboard: string;
    somethingWentWrong: string;
    unexpectedError: string;
    pageNotFound: string;
    all: string;
    actions: string;
    status: string;
    name: string;
    email: string;
    createdAt: string;
    loading: string;
    noResults: string;
    working: string;
    jsonPayload: string;
    table: {
      paginationAria: string;
    };
    pagination: {
      summary: string;
    };
  };
  dashboard: {
    title: string;
    subtitle: string;
    activeTenants: string;
    trialTenants: string;
    pastDue: string;
    suspended: string;
    mrr: string;
    healthMatrix: string;
    healthMatrixHint: string;
    healthUptime: string;
    healthUptimeHint: string;
    tenantDistribution: string;
    tenantDistributionHint: string;
    totalTenants: string;
    uptime24h: string;
    recentFraud: string;
    fraudHint: string;
    events: string;
    createTenant: string;
    viewHealth: string;
  };
  tenants: {
    title: string;
    new: string;
    suspend: string;
    reactivate: string;
    offboard: string;
    overview: string;
    billing: string;
    settings: string;
    domains: string;
    audit: string;
    workforce: string;
    suspendReason: string;
    changePlan: string;
  };
  users: {
    title: string;
    disable: string;
    enable: string;
    resetPassword: string;
    impersonate: string;
    impersonateReason: string;
    impersonateReasonPlaceholder: string;
    impersonateDialogBody: string;
    impersonateHint: string;
    impersonateOpened: string;
    impersonateFailed: string;
    profile: string;
    actions: string;
    email: string;
    tenant: string;
    role: string;
    status: string;
    active: string;
    inactive: string;
  };
  billing: {
    title: string;
    runDunning: string;
    description: string;
  };
  fraud: {
    title: string;
    resolve: string;
    addBlocklist: string;
  };
  domains: {
    title: string;
    forceVerify: string;
  };
  health: {
    title: string;
    subtitle: string;
    history: string;
    historyHint: string;
  };
  maintenance: {
    title: string;
    description: string;
    scopeHint: string;
    schedule: string;
    message: string;
    startsAt: string;
    endsAt: string;
    tenantOptional: string;
  };
  audit: {
    title: string;
    action: string;
    actor: string;
    tenant: string;
  };
};

type DotPrefix<T extends string, U extends string> = T extends '' ? U : `${T}.${U}`;

type DotPaths<T, Prev extends string = ''> = {
  [K in keyof T & string]: T[K] extends Record<string, unknown>
    ? DotPaths<T[K], DotPrefix<Prev, K>>
    : DotPrefix<Prev, K>;
}[keyof T & string];

export type MessageKey = DotPaths<Messages>;
