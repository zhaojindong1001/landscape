export class WifiServiceConfig {
  iface_name: string;
  enable: boolean;
  config: string;
  update_at?: number;

  constructor(obj?: {
    iface_name: string;
    enable?: boolean;
    config?: string;
    update_at?: number;
  }) {
    this.iface_name = obj?.iface_name ?? "";
    this.enable = obj?.enable ?? true;
    this.config = obj?.config ?? "";
    this.update_at = obj?.update_at;
  }
}
