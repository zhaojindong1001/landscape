export class IPV6PDServiceConfig {
  iface_name: string;
  enable: boolean;
  config: IPV6PDConfig;
  update_at?: number;

  constructor(obj: {
    iface_name: string;
    enable?: boolean;
    config?: IPV6PDConfig;
    update_at?: number;
  }) {
    this.iface_name = obj?.iface_name ?? "";
    this.enable = obj?.enable ?? true;
    this.config = new IPV6PDConfig(obj?.config ?? {});
    this.update_at = obj?.update_at;
  }
}

export class IPV6PDConfig {
  mac: string;

  constructor(obj?: { mac?: string }) {
    this.mac = obj?.mac ?? "";
  }
}
