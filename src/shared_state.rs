use mercator_db::DataBase;
use parser::Executor;
use parser::FiltersParser;
use parser::QueryParser;
use parser::Validator;

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

    pub fn filter(
        &self,
        input: &str,
        core: &str,
        output_space: Option<&str>,
        threshold_volume: Option<f64>,
    ) -> mercator_db::ResultSet {
        let parser = self.filter_parser();
        let parse;

        // Parse Input
        {
            info_time!("Parsing");
            parse = parser.parse(input);
        }
        match parse {
            Err(e) => {
                debug!("Parsing failed: \n{:?}", e);
                Err(format!("{}", e))
            }
            Ok(tree) => {
                let validation;
                let execution;

                // Check type coherence & validate tree
                {
                    info_time!("Type check");
                    validation = tree.validate();
                }
                if validation.is_err() {
                    debug!("Type check failed");
                    return Err("Type check failed".to_string());
                }

                // Execute filter.
                {
                    info_time!("Execution");
                    execution = tree.execute(self.db(), core, output_space, threshold_volume);
                }
                match execution {
                    Err(e) => {
                        debug!("Parsing failed: \n{:?}", e);
                        Err(e.to_string())
                    }
                    results @ Ok(_) => results,
                }
            }
        }
    }

    pub fn query(
        &self,
        input: &str,
        core: &str,
        output_space: Option<&str>,
        threshold_volume: Option<f64>,
    ) -> mercator_db::ResultSet {
        let parser = self.query_parser();
        let parse;

        // Parse Input
        {
            info_time!("Parsing");
            parse = parser.parse(input);
        }
        match parse {
            Err(e) => {
                debug!("Parsing failed: \n{:?}", e);
                Err(e.to_string())
            }
            Ok(None) => Ok(vec![]),
            Ok(Some(tree)) => {
                let validation;
                let execution;

                // Check type coherence & validate tree
                {
                    info_time!("Type check");
                    validation = tree.validate();
                }
                if validation.is_err() {
                    debug!("Type check failed");
                    return Err("Type check failed".to_string());
                }

                // Execute filter.
                {
                    info_time!("Execution");
                    execution = tree.execute(self.db(), core, output_space, threshold_volume);
                }
                match execution {
                    Err(e) => {
                        debug!("Parsing failed: \n{:?}", e);
                        Err(e.to_string())
                    }
                    results @ Ok(_) => results,
                }
            }
        }
    }
}
