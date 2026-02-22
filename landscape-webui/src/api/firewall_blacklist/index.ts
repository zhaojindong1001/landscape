import axiosService from "@/api";
import { FirewallBlacklistConfig } from "landscape-types/common/firewall_blacklist";

export async function get_firewall_blacklists(): Promise<
  FirewallBlacklistConfig[]
> {
  let data = await axiosService.get(`config/firewall_blacklists`);
  return data.data;
}

export async function get_firewall_blacklist(
  id: string,
): Promise<FirewallBlacklistConfig> {
  let data = await axiosService.get(`config/firewall_blacklists/${id}`);
  return data.data;
}

export async function push_firewall_blacklist(
  config: FirewallBlacklistConfig,
): Promise<void> {
  await axiosService.post(`config/firewall_blacklists`, config);
}

export async function delete_firewall_blacklist(id: string): Promise<void> {
  await axiosService.delete(`config/firewall_blacklists/${id}`);
}
