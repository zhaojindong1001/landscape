export class FirewallServiceConfig {
  iface_name: string;
  enable: boolean;
  update_at?: number;

  constructor(obj?: {
    iface_name: string;
    enable?: boolean;
    update_at?: number;
  }) {
    this.iface_name = obj?.iface_name ?? "";
    this.enable = obj?.enable ?? true;
    this.update_at = obj?.update_at;
  }
}
