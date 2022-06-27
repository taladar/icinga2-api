//! The Icinga2 API filter language used both when querying and when targeting actions

trait IcingaFilterExpression {
    fn write_filter_expression<F: std::fmt::Write>(&self, f: &mut F) -> Result<(), crate::error::Error>;
}

/// a filter expression returning a boolean result
#[derive(Debug)]
pub enum BooleanFilterExpression<Bindings> {
    /// a literal boolean value
    Literal(bool),
    /// a variable expansion
    Variable {
        /// the actual variable (possibly with element accesses)
        inner: Box<VariableFilterExpression<Bindings>>,
    },
    /// a boolean and operation
    And {
        /// the value on the left of the and
        left: Box<BooleanFilterExpression<Bindings>>,
        /// the value on the right of the and
        right: Box<BooleanFilterExpression<Bindings>>,
    },
    /// a boolean or operation
    Or {
        /// the value on the left of the or
        left: Box<BooleanFilterExpression<Bindings>>,
        /// the value on the right of the or
        right: Box<BooleanFilterExpression<Bindings>>,
    },
    /// a boolean not operation
    Not {
        /// the value negated by the not
        inner: Box<BooleanFilterExpression<Bindings>>,
    },
    /// an equality comparison of two string expressions
    StringEqualComparison {
        /// the value on the left of the comparison
        left: Box<StringFilterExpression<Bindings>>,
        /// the value on the right of the comparison
        right: Box<StringFilterExpression<Bindings>>,
    },
    /// an inequality comparison of two string expressions
    StringNotEqualComparison {
        /// the value on the left of the comparison
        left: Box<StringFilterExpression<Bindings>>,
        /// the value on the right of the comparison
        right: Box<StringFilterExpression<Bindings>>,
    },
}

impl<Bindings> IcingaFilterExpression for BooleanFilterExpression<Bindings>
  where Bindings: IcingaFilterExpression
  {
    fn write_filter_expression<F: std::fmt::Write>(&self, f: &mut F) -> Result<(), crate::error::Error> {
        match self {
            BooleanFilterExpression::Literal(b) => write!(f, "{}", b).map_err(crate::error::Error::WritingFilterExpression),
            BooleanFilterExpression::Variable { inner } => inner.write_filter_expression(f),
            BooleanFilterExpression::And { left, right } => {
                left.write_filter_expression(f)?;
                write!(f, " && ").map_err(crate::error::Error::WritingFilterExpression)?;
                right.write_filter_expression(f)?;
                Ok(())
            },
            BooleanFilterExpression::Or { left, right } => {
                left.write_filter_expression(f)?;
                write!(f, " || ").map_err(crate::error::Error::WritingFilterExpression)?;
                right.write_filter_expression(f)?;
                Ok(())
            },
            BooleanFilterExpression::Not { inner } => {
                write!(f, "!").map_err(crate::error::Error::WritingFilterExpression)?;
                inner.write_filter_expression(f)?;
                Ok(())
            }
            BooleanFilterExpression::StringEqualComparison { left, right } => {
                left.write_filter_expression(f)?;
                write!(f, " == ").map_err(crate::error::Error::WritingFilterExpression)?;
                right.write_filter_expression(f)?;
                Ok(())
            }
            BooleanFilterExpression::StringNotEqualComparison { left, right } => {
                left.write_filter_expression(f)?;
                write!(f, " != ").map_err(crate::error::Error::WritingFilterExpression)?;
                right.write_filter_expression(f)?;
                Ok(())
            }
        }
    }
}

/// a filter expression returning a string result
#[derive(Debug)]
pub enum StringFilterExpression<Bindings> {
    /// a string literal
    Literal(String),
    /// a variable expansion
    Variable {
        /// the actual variable (possibly with element accesses)
        inner: Box<VariableFilterExpression<Bindings>>,
    },
}

impl<Bindings> IcingaFilterExpression for StringFilterExpression<Bindings>
  where Bindings: IcingaFilterExpression
  {
    fn write_filter_expression<F: std::fmt::Write>(&self, f: &mut F) -> Result<(), crate::error::Error> {
        match self {
            StringFilterExpression::Literal(s) => write!(f, "\"{}\"", s).map_err(crate::error::Error::WritingFilterExpression),
            StringFilterExpression::Variable { inner } => {
                inner.write_filter_expression(f)
            },
        }
    }
}

/// a filter expression returning an array result
#[derive(Debug)]
pub enum ArrayFilterExpression<Bindings> {
    /// a literal array of string filter expressions
    LiteralStrings(Vec<StringFilterExpression<Bindings>>),
    /// a variable expansion
    Variable {
        /// the actual variable (possibly with element accesses)
        inner: Box<VariableFilterExpression<Bindings>>,
    },
}

impl<Bindings> IcingaFilterExpression for ArrayFilterExpression<Bindings>
  where Bindings: IcingaFilterExpression
  {
    fn write_filter_expression<F: std::fmt::Write>(&self, f: &mut F) -> Result<(), crate::error::Error> {
        match self {
            ArrayFilterExpression::LiteralStrings(xs) => {
                let mut first = true;
                write!(f, "[").map_err(crate::error::Error::WritingFilterExpression)?;
                for x in xs {
                    if !first  {
                        write!(f, ", ").map_err(crate::error::Error::WritingFilterExpression)?;
                    } else {
                        first = false;
                    }
                    x.write_filter_expression(f)?;
                }
                write!(f, "]").map_err(crate::error::Error::WritingFilterExpression)?;
                Ok(())
            }
            ArrayFilterExpression::Variable { inner } => {
                inner.write_filter_expression(f)
            }
        }
    }
}

/// a variable in the filter language
#[derive(Debug)]
pub struct VariableFilterExpression<Bindings> {
    /// one of the bindings available in this type of query
    pub base: Bindings,
    /// the other elements after the initial binding (e.g. the name bit in host.name)
    pub elements: Vec<String>,
}

impl<Bindings> IcingaFilterExpression for VariableFilterExpression<Bindings>
  where Bindings: IcingaFilterExpression
  {
    fn write_filter_expression<F: std::fmt::Write>(&self, f: &mut F) -> Result<(), crate::error::Error> {
        self.base.write_filter_expression(f)?;
        for element in &self.elements {
            write!(f, ".{}", element).map_err(crate::error::Error::WritingFilterExpression)?;
        }
        Ok(())
    }
}

/// all the bindings in scope when doing service queries
#[derive(Debug)]
pub enum ServiceBindings {
    /// the host for the service
    Host,
    /// the check command for this service
    CheckCommand,
    /// the time period when this check is active
    CheckPeriod,
    /// the event command for this service
    EventCommand,
    /// the command point executing this service
    CommandEndpoint,
}

impl IcingaFilterExpression for ServiceBindings {
    fn write_filter_expression<F: std::fmt::Write>(&self, f: &mut F) -> Result<(), crate::error::Error> {
        match self {
            ServiceBindings::Host => write!(f, "host").map_err(crate::error::Error::WritingFilterExpression),
            ServiceBindings::CheckCommand => write!(f, "check_command").map_err(crate::error::Error::WritingFilterExpression),
            ServiceBindings::CheckPeriod => write!(f, "check_period").map_err(crate::error::Error::WritingFilterExpression),
            ServiceBindings::EventCommand => write!(f, "event_command").map_err(crate::error::Error::WritingFilterExpression),
            ServiceBindings::CommandEndpoint => write!(f, "command_endpoint").map_err(crate::error::Error::WritingFilterExpression),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::error::Error;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_expression() -> Result<(), Box<dyn Error>> {
        let expr: BooleanFilterExpression<ServiceBindings> = BooleanFilterExpression::And {
            left: Box::new(BooleanFilterExpression::Literal(true)),
            right: Box::new(BooleanFilterExpression::StringEqualComparison {
                left: Box::new(StringFilterExpression::Literal("foo".to_string())),
                right: Box::new(StringFilterExpression::Variable {
                    inner: Box::new(VariableFilterExpression {
                        base: ServiceBindings::Host,
                        elements: vec!["name".to_string()],
                    }),
                }),
            }),
        };
        let mut s = String::new();
        expr.write_filter_expression(&mut s)?;
        assert_eq!(s, "true && \"foo\" == host.name");

        Ok(())
    }
}

// TODO: filters https://icinga.com/docs/icinga-2/latest/doc/12-icinga2-api/#advanced-filters (operations, functions,.. below are just a selection of the most immediately interesting ones)
// * what are the semantics of a variable that does not exist (e.g. typo, field access to custom variables)
// * what are the semantics of a type mismatch (e.g. you apply string functions to a custom variable or field that is an array)
// * boolean literals
// * numeric literals (floating point numbers and integers are one type in icinga)
// * string literals (do filters support multi-line string literals?)
// * enum literals (service and host state and state type in particular)
// * duration literals
// * null literal
// * dictionary literals
// * array literals
// * operators ( https://icinga.com/docs/icinga-2/latest/doc/17-language-reference/#operators )
// ** () grouping
// ** function call
// ** element access (can we somehow get validation of field names here? Would require us to know the type of a variable but there is only a handful of those, could not be for all fields though since some are runtime, e.g. custom variables)
// ** logical not
// ** unary minus
// ** multiplication
// ** division
// ** remainder
// ** add numbers/durations
// ** concatenate string
// ** subtract numbers/durations
// ** equality
// ** inequality
// ** logical and
// ** logical or
// ** element in array
// ** element not in array
// ** less than, greater than, less than or equal, greater than or equal for numbers, durations (and strings?)
// * variables provided by the filter (varies by object type we query, some types of variables appear under different names for different queries)
// * functions ( https://icinga.com/docs/icinga-2/latest/doc/18-library-reference/ )
// ** match
// ** regex
// ** intersection
// ** union
// ** range
// ** get_time
// ** Math.min
// ** Math.max
// ** Array.all
// ** Array.any
// ** Array.contains
// ** Dictionary.contains
// ** Dictionary.keys
// ** Dictionary.values
// ** String.contains
// ** String.split
// ** String.trim
// ** String.lower
// ** String.upper
