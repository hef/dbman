use serde::{Deserialize, Serialize};

use crate::{Database, Error};
use crate::Error::HeritageValidation;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Heritage {
    heritage: String,
    owner: Option<String>,
    resource: Option<String>,
    namespace: Option<String>,
}

impl Heritage {
    pub fn builder() -> HeritageBuilder {
        HeritageBuilder {
            owner: None,
            resource: None,
            namespace: None,
        }
    }

    pub fn validate(&self, z: &str) -> Result<(), Error> {
        let compare = serde_json::from_str::<Heritage>(&z).map_err(|e| {})?;
        if self.heritage != compare.heritage {
            Err(HeritageValidation("".into(), "heritage".into(), "dbman".into(), compare.heritage.into()))
        }
        if self.owner != compare.owner {
            Err(HeritageValidation("".into(), "owner".into(), self.owner.into(), compare.owner.into()))
        }

        Ok(())
    }
}

pub(crate) struct HeritageBuilder {
    owner: Option<String>,
    resource: Option<String>,
    namespace: Option<String>,
}

impl HeritageBuilder {
    pub fn owner(&mut self, owner: String) -> &mut Self {
        self.owner = Some(owner);
        self
    }

    pub fn resource(&mut self, db: &Database) -> &mut Self {
        self.resource = db.metadata.name.clone();
        self.namespace = db.metadata.namespace.clone();
        self
    }

    pub fn build(&self) -> Heritage {
        Heritage {
            heritage: "dbman".into(),
            owner: Some(self.owner.clone().unwrap_or("default".into())),
            resource: self.resource.to_owned(),
        }
    }
}
