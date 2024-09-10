use tonic::{transport::Server, Request, Response, Status};
use sbv2_core::tts::TTSModel;
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod tts_service {
    tonic::include_proto!("tts");
}

use tts_service::tts_server::{Tts, TtsServer};
use tts_service::{SynthesizeRequest, SynthesizeResponse};

struct TtsService {
    tts_model: Arc<Mutex<Option<TTSModel>>>,
}

#[tonic::async_trait]
impl Tts for TtsService {
    async fn synthesize(
        &self,
        request: Request<SynthesizeRequest>,
    ) -> Result<Response<SynthesizeResponse>, Status> {
        let text = request.into_inner().text;
        let buffer = {
            let mut tts_model = self.tts_model.lock().await;
            let tts_model = if let Some(tts_model) = &*tts_model {
                tts_model
            } else {
                *tts_model = Some(TTSModel::new(
                    &env::var("BERT_MODEL_PATH").map_err(|_| Status::internal("Failed to get BERT_MODEL_PATH"))?,
                    &env::var("MAIN_MODEL_PATH").map_err(|_| Status::internal("Failed to get MAIN_MODEL_PATH"))?,
                    &env::var("STYLE_VECTORS_PATH").map_err(|_| Status::internal("Failed to get STYLE_VECTORS_PATH"))?,
                ).map_err(|_| Status::internal("Failed to initialize TTSModel"))?);
                tts_model.as_ref().unwrap()
            };
            let (bert_ori, phones, tones, lang_ids) = tts_model.parse_text(&text)
                .map_err(|_| Status::internal("Failed to parse text"))?;
            let style_vector = tts_model.get_style_vector(0, 1.0)
                .map_err(|_| Status::internal("Failed to get style vector"))?;
            tts_model.synthesize(bert_ori.to_owned(), phones, tones, lang_ids, style_vector)
                .map_err(|_| Status::internal("Failed to synthesize audio"))?
        };

        Ok(Response::new(SynthesizeResponse { audio: buffer }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let addr = "0.0.0.0:50051".parse()?;
    let tts_service = TtsService {
        tts_model: Arc::new(Mutex::new(None)),
    };

    Server::builder()
        .add_service(TtsServer::new(tts_service))
        .serve(addr)
        .await?;

    Ok(())
}