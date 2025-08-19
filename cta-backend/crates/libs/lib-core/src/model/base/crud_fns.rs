use modql::filter::ListOptions;

use crate::model::{base::LIST_LIMIT_MAX, Error, Result};

pub fn compute_list_options(list_options: Option<ListOptions>) -> Result<ListOptions> {
    if let Some(mut list_options) = list_options {
        // validate the limit.
        if let Some(limit) = list_options.limit {
            if limit > LIST_LIMIT_MAX {
                return Err(Error::ListLimitOverMax {
                    max: LIST_LIMIT_MAX,
                    actual: limit,
                });
            }
        }
        // Set the default limit if no limit
        else {
            list_options.limit = Some(LIST_LIMIT_MAX);
        }

        Ok(list_options)
    }
    // When None, return default
    else {
        Ok(ListOptions {
            limit: Some(LIST_LIMIT_MAX),
            offset: None,
            order_bys: Some("id".into()),
        })
    }
}
