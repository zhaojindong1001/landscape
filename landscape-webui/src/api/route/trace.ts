import axiosService from "@/api";
import type {
  FlowMatchRequest,
  FlowMatchResult,
  FlowVerdictRequest,
  FlowVerdictResult,
} from "landscape-types/common/route_trace";

export async function trace_flow_match(
  req: FlowMatchRequest,
): Promise<FlowMatchResult> {
  const data = await axiosService.post("services/route/trace/flow_match", req);
  return data.data;
}

export async function trace_verdict(
  req: FlowVerdictRequest,
): Promise<FlowVerdictResult> {
  const data = await axiosService.post("services/route/trace/verdict", req);
  return data.data;
}
