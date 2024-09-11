import type * as grpc from '@grpc/grpc-js';
import type { MessageTypeDefinition } from '@grpc/proto-loader';

import type { TtsClient as _tts_TtsClient, TtsDefinition as _tts_TtsDefinition } from './tts/Tts';

type SubtypeConstructor<Constructor extends new (...args: any) => any, Subtype> = {
  new(...args: ConstructorParameters<Constructor>): Subtype;
};

export interface ProtoGrpcType {
  tts: {
    SynthesizeRequest: MessageTypeDefinition
    SynthesizeResponse: MessageTypeDefinition
    Tts: SubtypeConstructor<typeof grpc.Client, _tts_TtsClient> & { service: _tts_TtsDefinition }
  }
}

