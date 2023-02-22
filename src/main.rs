
trait Display {
    fn display(&self) -> String;
}

#[derive(Debug)]
struct Symbol {
    text: &'static str,
}

impl Symbol {
    fn new(text: &'static str) -> Self {
        if text.len() == 0 {
            panic!("Symbol text must be nonempty string.");
        }

        Symbol {
            text
        }
    }

    fn is_terminal(&self) -> bool {
        return self.text.bytes().nth(0).unwrap() != ('<' as u8);
    }
}

#[derive(Debug)]
struct Production {
    symbols: Vec<Symbol>,
}

impl Production {
    fn new(symbols: Vec<Symbol>) -> Self {
        Production {
            symbols
        }
    }
}

impl Display for Production {
    fn display(&self) -> String {
        let mut result = String::new();
        let symbs = self.symbols.iter();
        for s in symbs {
            result.push_str(s.text);
        }
        return result;
    }
}

#[derive(Debug)]
struct Rule {
    symbol: Symbol,
    derivations: Vec<Production>,
}

impl Display for Vec<Production> {
    fn display(&self) -> String {
        let mut result = String::new();
        let prods = self.iter().enumerate();
        let num_prods = self.len();
        for (i, p) in prods {
            result.push_str(&p.display());
            if i < num_prods - 1 {
                result.push_str(&" or ");
            }
        }

        return result;
    }
}

impl Rule {
    fn new(symbol: &'static str, productions: Vec<Production>) -> Self {
        let lhs_symbol = Symbol::new(symbol);
        if lhs_symbol.is_terminal() {
            panic!("LHS symbol of rule must be non-terminal.");
        }

        if productions.len() == 0 {
            panic!("Rule must have at least one production.");
        }

        Rule {
            symbol: lhs_symbol,
            derivations: productions,
        }
    }

    fn has_direct_left_recursion(&self) -> bool {
        self.derivations.iter().fold(false, |acc, p| {
            acc || (p.symbols.iter().nth(0).unwrap().text == self.symbol.text)
        })
    }
}

struct Grammar {
    rules: Vec<Rule>,
}

impl Grammar {
    fn new(rules: Vec<Rule>) -> Self {
        Grammar {
            rules,
        }
    }

    fn derives_to_symbol_helper(&self, start: &Symbol, target: &Symbol) -> bool {
        if start.text == target.text {
            return true;
        }

        return self.derives_to_symbol(start, target);
    }

    fn derives_to_symbol(&self, start: &Symbol, target: &Symbol) -> bool {
        if start.is_terminal() {
            return false;
        }

        // Find start symbol in rules
        let starting_rule = self.rules.iter().find(|rule| {
            rule.symbol.text == start.text
        });

        let starting_rule = starting_rule.expect("No matching LHS symbol.");

        return starting_rule.derivations.iter().fold(false, |acc, production| {
            return acc || self.derives_to_symbol_helper(production.symbols.iter().nth(0).unwrap(), target);
        });
    }

    fn has_indirect_left_recursion(&self) -> bool {
        self.rules.iter().fold(false, |acc, r| {
            return acc || self.derives_to_symbol(&r.symbol, &r.symbol);
        })
    }

    fn has_left_recursion(&self) -> bool {
        let has_direct_left = self.rules.iter().fold(false, |acc, r| {
            acc || r.has_direct_left_recursion()
        });

        if has_direct_left {
            return true;
        }

        return self.has_indirect_left_recursion();
    }


}

impl Display for Grammar {
    fn display(&self) -> String {
        let longest: usize = self.rules.iter().reduce(|acc, rule| {
            if rule.symbol.text.len() > acc.symbol.text.len() {
                return rule;
            }
            return acc;
        }).unwrap().symbol.text.len();

        let mut result = String::new();

        for rule in self.rules.iter() {
            let suffix_spaces_count: usize = (longest - rule.symbol.text.len()) + 1;
            result.push_str(rule.symbol.text);
            result.push_str(&" ".repeat(suffix_spaces_count));
            result.push_str(":= ");
            result.push_str(&rule.derivations.display());
            result.push_str("\n");
        }

        return result;
    }
}

fn main() {
    let g = Grammar::new(vec![
        Rule::new("<REGEX>", vec![
            Production::new(vec![
                Symbol::new("<LOW_PRECEDENCE>"),
            ]),
        ]),
        Rule::new("<LOW_PRECEDENCE>", vec![
            Production::new(vec![
                Symbol::new("<MED_PRECEDENCE>"),
                Symbol::new("<ALTERNAT>"),
            ]),
        ]),
        Rule::new("<ALTERNAT>", vec![
            Production::new(vec![
                Symbol::new("|"),
                Symbol::new("<LOW_PRECEDENCE>"),
            ]),
            Production::new(vec![
                Symbol::new("EmptyString"),
            ]),
        ]),
        Rule::new("<MED_PRECEDENCE>", vec![
            Production::new(vec![
                Symbol::new("<HIGH_PRECEDENCE>"),
                Symbol::new("<CONCAT>"),
            ]),
        ]),
        Rule::new("<CONCAT>", vec![
            Production::new(vec![
                Symbol::new("<MED_PRECEDENCE>"),
            ]),
            Production::new(vec![
                Symbol::new("EmptyString"),
            ]),
        ]),
        Rule::new("<HIGH_PRECEDENCE>", vec![
            Production::new(vec![
                Symbol::new("<GIGA_PRECEDENCE>"),
                Symbol::new("<KLEENE>"),
            ]),
        ]),
        Rule::new("<KLEENE>", vec![
            Production::new(vec![
                Symbol::new("*"),
            ]),
            Production::new(vec![
                Symbol::new("EmptyString"),
            ]),
        ]),
        Rule::new("<GIGA_PRECEDENCE>", vec![
            Production::new(vec![
                Symbol::new("("),
                Symbol::new("<LOW_PRECEDENCE>"),
                Symbol::new(")"),
            ]),
            Production::new(vec![
                Symbol::new("<TERMINAL>"),
            ]),
        ]),
        Rule::new("<TERMINAL>", vec![
            Production::new(vec![
                Symbol::new("EmptySet"),
            ]),
            Production::new(vec![
                Symbol::new("EmptyString"),
            ]),
            Production::new(vec![
                Symbol::new("C"),
            ]),
        ]),
    ]);

    println!("{}", g.display());
    println!("Has left recursion? {}", g.has_left_recursion());
}
