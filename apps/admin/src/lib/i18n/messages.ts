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
  };
  common: {
    previous: string;
    next: string;
    cancel: string;
    confirm: string;
    working: string;
  };
};

export type MessageKey =
  | `nav.${keyof Messages['nav']}`
  | `auth.${keyof Messages['auth']}`
  | `shell.${keyof Messages['shell']}`
  | `common.${keyof Messages['common']}`;
