// Original file: ../../sbv2_grpc/proto/tts.proto

import type * as grpc from '@grpc/grpc-js'
import type { MethodDefinition } from '@grpc/proto-loader'
import type { SynthesizeRequest as _tts_SynthesizeRequest, SynthesizeRequest__Output as _tts_SynthesizeRequest__Output } from '../tts/SynthesizeRequest';
import type { SynthesizeResponse as _tts_SynthesizeResponse, SynthesizeResponse__Output as _tts_SynthesizeResponse__Output } from '../tts/SynthesizeResponse';

export interface TtsClient extends grpc.Client {
  Synthesize(argument: _tts_SynthesizeRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_tts_SynthesizeResponse__Output>): grpc.ClientUnaryCall;
  Synthesize(argument: _tts_SynthesizeRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_tts_SynthesizeResponse__Output>): grpc.ClientUnaryCall;
  Synthesize(argument: _tts_SynthesizeRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_tts_SynthesizeResponse__Output>): grpc.ClientUnaryCall;
  Synthesize(argument: _tts_SynthesizeRequest, callback: grpc.requestCallback<_tts_SynthesizeResponse__Output>): grpc.ClientUnaryCall;
  synthesize(argument: _tts_SynthesizeRequest, metadata: grpc.Metadata, options: grpc.CallOptions, callback: grpc.requestCallback<_tts_SynthesizeResponse__Output>): grpc.ClientUnaryCall;
  synthesize(argument: _tts_SynthesizeRequest, metadata: grpc.Metadata, callback: grpc.requestCallback<_tts_SynthesizeResponse__Output>): grpc.ClientUnaryCall;
  synthesize(argument: _tts_SynthesizeRequest, options: grpc.CallOptions, callback: grpc.requestCallback<_tts_SynthesizeResponse__Output>): grpc.ClientUnaryCall;
  synthesize(argument: _tts_SynthesizeRequest, callback: grpc.requestCallback<_tts_SynthesizeResponse__Output>): grpc.ClientUnaryCall;
  
}

export interface TtsHandlers extends grpc.UntypedServiceImplementation {
  Synthesize: grpc.handleUnaryCall<_tts_SynthesizeRequest__Output, _tts_SynthesizeResponse>;
  
}

export interface TtsDefinition extends grpc.ServiceDefinition {
  Synthesize: MethodDefinition<_tts_SynthesizeRequest, _tts_SynthesizeResponse, _tts_SynthesizeRequest__Output, _tts_SynthesizeResponse__Output>
}
