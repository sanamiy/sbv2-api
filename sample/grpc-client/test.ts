import * as grpc from "@grpc/grpc-js";
import * as protoLoader from "@grpc/proto-loader";
import { ProtoGrpcType } from "./proto/tts";
import { TtsClient } from "./proto/tts/Tts";
import { SynthesizeRequest } from "./proto/tts/SynthesizeRequest";
import { SynthesizeResponse } from "./proto/tts/SynthesizeResponse";
import { promises as fsPromises } from "fs";
const PROTO_PATH = "../../sbv2_grpc/proto/tts.proto";

const packageDefinition = protoLoader.loadSync(PROTO_PATH);
const protoDescriptor = grpc.loadPackageDefinition(
  packageDefinition
) as unknown as ProtoGrpcType;

const client = new protoDescriptor.tts.Tts(
  "localhost:50051",
  grpc.credentials.createInsecure()
) as TtsClient;

function synthesize(text: string): Promise<Buffer> {
  return new Promise((resolve, reject) => {
    const request: SynthesizeRequest = { text };
    client.Synthesize(request, (error, response) => {
      if (error) {
        reject(error);
      } else if (response && response.audio) {
        resolve(response.audio);
      } else {
        reject(new Error("No response or audio received"));
      }
    });
  });
}

// 使用例
async function main() {
  try {
    const audioBuffer = await synthesize("こんにちは、世界！");
    console.log("Audio synthesized, length:", audioBuffer.length);
    await fsPromises.writeFile("output.wav", audioBuffer);
  } catch (error) {
    console.error("Synthesis failed:", error);
  }
}

main();
