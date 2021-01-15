use tonic::{
    Request,
    Response,
    Status,
    Streaming,
};

use tokio::sync::mpsc;

use std::io;
use std::sync::{
    Arc,
    Mutex,
};
use std::result::Result;

use crate::ngt::NGT;

pub use vald::v1::insert_server::{
    Insert,
    InsertServer,
};

pub use vald::v1::search_server::{
    Search,
    SearchServer,
};

pub use self::core::v1::agent_server:: {
    Agent,
    AgentServer,
};

pub mod vald {
    pub mod v1 {
        tonic::include_proto!("vald.v1");
    }
}

pub mod core {
    pub mod v1 {
        tonic::include_proto!("core.v1");
    }
}

pub mod errors {
    pub mod v1 {
        tonic::include_proto!("errors.v1");
    }
}

pub mod payload {
    pub mod v1 {
        tonic::include_proto!("payload.v1");
    }
}

#[derive(Debug, Default)]
pub struct ValdImpl {
    ngt: Arc<Mutex<NGT>>,
}

impl ValdImpl {
    pub fn initialize(&self) -> Result<(), io::Error> {
        &self.ngt.lock().unwrap().initialize().unwrap();

        Ok(())
    }
}

impl Clone for ValdImpl {
    fn clone(&self) -> ValdImpl {
        ValdImpl {
            ngt: self.ngt.clone(),
        }
    }
}

#[tonic::async_trait]
impl Insert for ValdImpl {
    async fn insert(
        &self,
        request: Request<payload::v1::insert::Request>,
    ) -> Result<Response<payload::v1::object::Location>, Status> {
        let msg = request.get_ref();
        let obj = match &msg.vector {
            Some(o) => o,
            None => return Err(Status::invalid_argument("vector is required.")),
        };
        let name = obj.id.clone();
        let vector = obj.vector.clone();

        let uuid = match &self.ngt.lock().unwrap().insert(&name, vector){
            Ok(oid) => oid.to_string(),
            Err(err) => return Err(Status::internal(err.to_string())),
        };

        let reply = payload::v1::object::Location{
            name,
            uuid,
            ips: vec!["192.168.1.1".to_string()],
        };

        Ok(Response::new(reply))
    }

    type StreamInsertStream = mpsc::Receiver<Result<payload::v1::object::StreamLocation, Status>>;

    async fn stream_insert(
        &self,
        request: Request<Streaming<payload::v1::insert::Request>>,
    ) -> Result<Response<Self::StreamInsertStream>, Status> {
        unimplemented!()
    }

    async fn multi_insert(
        &self,
        request: Request<payload::v1::insert::MultiRequest>,
        ) -> Result<Response<payload::v1::object::Locations>, Status> {
        unimplemented!()
    }
}

#[tonic::async_trait]
impl Search for ValdImpl {
    async fn search(
        &self,
        request: Request<payload::v1::search::Request>,
    ) -> Result<Response<payload::v1::search::Response>, Status> {
        let msg = request.get_ref();
        let vector: Vec<f64> = msg.vector.iter().map(|f| {
            *f as f64
        }).collect();
        let config = match &msg.config {
            Some(c) => c,
            None => return Err(Status::invalid_argument("config is required.")),
        };

        let request_id = config.request_id.clone();
        let num: u64 = From::from(config.num);

        let results = self.ngt.lock().unwrap().search(vector, num, config.epsilon).unwrap();

        let reply = payload::v1::search::Response{
            request_id,
            results,
        };

        Ok(Response::new(reply))
    }

    async fn search_by_id(
        &self,
        request: Request<payload::v1::search::IdRequest>,
    ) -> Result<Response<payload::v1::search::Response>, Status> {
        unimplemented!()
    }

    type StreamSearchStream = mpsc::Receiver<Result<payload::v1::search::StreamResponse, Status>>;

    async fn stream_search(
        &self,
        request: Request<Streaming<payload::v1::search::Request>>,
    ) -> Result<Response<Self::StreamSearchStream>, Status> {
        unimplemented!()
    }

    type StreamSearchByIDStream = mpsc::Receiver<Result<payload::v1::search::StreamResponse, Status>>;

    async fn stream_search_by_id(
        &self,
        request: Request<Streaming<payload::v1::search::IdRequest>>,
    ) -> Result<Response<Self::StreamSearchByIDStream>, Status> {
        unimplemented!()
    }

    async fn multi_search(
        &self,
        request: Request<payload::v1::search::MultiRequest>,
    ) -> Result<Response<payload::v1::search::Responses>, Status> {
        unimplemented!()
    }

    async fn multi_search_by_id(
        &self,
        request: Request<payload::v1::search::MultiIdRequest>,
    ) -> Result<Response<payload::v1::search::Responses>,Status> {
        unimplemented!()
    }
}

#[tonic::async_trait]
impl Agent for ValdImpl {
    async fn create_index(
        &self,
        request: Request<payload::v1::control::CreateIndexRequest>,
    ) -> Result<Response<payload::v1::Empty>, Status> {
        &self.ngt.lock().unwrap().create_index();

        let res = payload::v1::Empty {};

        Ok(Response::new(res))
    }

    async fn save_index(
        &self,
        request: Request<payload::v1::Empty>,
    ) -> Result<Response<payload::v1::Empty>, Status> {
        unimplemented!()
    }

    async fn create_and_save_index(
        &self,
        request: Request<payload::v1::control::CreateIndexRequest>,
    ) -> Result<Response<payload::v1::Empty>, Status> {
        unimplemented!()
    }

    async fn index_info(
        &self,
        request: Request<payload::v1::Empty>,
    ) -> Result<Response<payload::v1::info::index::Count>,Status> {
        unimplemented!()
    }
}
