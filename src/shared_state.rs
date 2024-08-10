use mercator_db::CoreQueryParameters;
use mercator_db::DataBase;
use mercator_parser::Bag;
use mercator_parser::Executor;
use mercator_parser::FiltersParser;
use mercator_parser::Projection;
use mercator_parser::QueryParser;
use mercator_parser::Validator;

pub struct SharedState {
    db: DataBase,
    query_parser: QueryParser,
    filter_parser: FiltersParser,
}

impl SharedState {
    pub fn new(db: DataBase) -> Self {
        SharedState {
            db,
            query_parser: QueryParser::new(),
            filter_parser: FiltersParser::new(),
        }
    }

    pub fn db(&self) -> &DataBase {
        &self.db
    }

    pub fn filter_parser(&self) -> &FiltersParser {
        &self.filter_parser
    }

    pub fn query_parser(&self) -> &QueryParser {
        &self.query_parser
    }

    pub fn execute<'e, T>(
        &'e self,
        tree: &'e T, //&'e Bag,
        core: &'e str,
        parameters: &'e CoreQueryParameters<'e>,
    ) -> mercator_db::ResultSet<'e>
    where
        T: Executor<'e, ResultSet = mercator_db::ResultSet<'e>>,
    {
        // Execute filter.
        let execution = {
            info_time!("Execution");
            // _FIXME: Output space is defined as part of the projection
            //        and is ignored by projections operators.
            tree.execute(core, parameters)
        };

        match execution {
            Err(e) => {
                debug!("Execution failed: \n{:?}", e);
                Err(e)
            }
            results @ Ok(_) => results,
        }
    }

    pub fn filter<'q>(&'q self, filter: &'q str) -> Result<Bag, String> {
        let parser = self.filter_parser();
        let parse;

        // Parse Input
        {
            debug_time!("Parsing");
            parse = parser.parse(filter);
        }

        match parse {
            Err(e) => {
                debug!("Parsing failed: \n{:?}", e);
                Err(format!("{}", e))
            }
            Ok(tree) => {
                // Check type coherence & validate tree
                {
                    debug_time!("Type check");
                    let _ = tree.validate()?;
                }

                Ok(tree)
            }
        }
    }

    pub fn query(&self, query: &str) -> Result<Projection, String> {
        let parser = self.query_parser();
        let parse;

        // Parse Input
        {
            debug_time!("Parsing");
            parse = parser.parse(query);
        }
        match parse {
            Err(e) => {
                debug!("Parsing failed: \n{:?}", e);
                Err(e.to_string())
            }
            Ok(None) => Err("Query is empty!".to_string()),
            Ok(Some(tree)) => {
                // Check type coherence & validate tree
                {
                    debug_time!("Type check");
                    let _ = tree.validate()?;
                }

                Ok(tree)
            }
        }
    }
}
