use serde::{Deserialize, Serialize};

use crate::{Database, Error};
use crate::Error::HeritageValidation;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Heritage {
    heritage: String,
    resource: String,
    namespace: String,
}

impl Heritage {
    pub fn builder() -> HeritageBuilder {
        HeritageBuilder {
            resource: None,
            namespace: None,
        }
    }
    pub fn validate(&self, database: &str, heritage_string: &str) -> Result<(), Error> {
        let compare = serde_json::from_str::<Heritage>(&heritage_string).map_err(|e|
            Error::FailedToDeserializeHeritage(Box::new(e), database.into())
        )?;
        if self.heritage != compare.heritage {
            return Err(HeritageValidation("".into(), "heritage".into(), "dbman".into(), compare.heritage.into()))
        }
        if self.namespace != compare.namespace {
            return Err(HeritageValidation( database.into(), "namespace".into(), self.namespace.into(), compare.namespace))
        }
        if self.resource != compare.resource {
            return Err(HeritageValidation( database.into(), "resource".into(), self.resource.into(), compare.resource))
        }
        Ok(())
    }
}

pub(crate) struct HeritageBuilder {
    resource: Option<String>,
    namespace: Option<String>,
}

impl HeritageBuilder {
    pub fn resource(&mut self, db: &Database) -> &mut Self {
        self.resource = db.metadata.name.clone();
        self.namespace = db.metadata.namespace.clone();
        self
    }

    pub fn build(&self) -> Heritage {
        Heritage {
            heritage: "dbman".into(),
            resource: self.resource.unwrap_or("".into()),
            namespace: self.namespace.unwrap_or("".into()),
        }
    }
}
