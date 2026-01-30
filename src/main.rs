use clap::{Parser, Subcommand};
use ls::{equity, debt, trade, feed};
use serde_json::json;

#[derive(Parser)]
#[command(name = "ls")]
#[command(about = "Launchpad Skill - Agentic Company Management CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Output in JSON format
    #[arg(long, global = true)]
    json: bool,
}

#[derive(Subcommand)]
enum Commands {
    Equity {
        #[command(subcommand)]
        action: EquityCommands,
    },
    Debt {
        #[command(subcommand)]
        action: DebtCommands,
    },
    Trade {
        #[command(subcommand)]
        action: TradeCommands,
    },
    Feed {
        /// Address to view narrative for
        address: String,
    },
}

#[derive(Subcommand)]
enum EquityCommands {
    Issue {
        #[arg(long)]
        name: String,
        #[arg(long)]
        symbol: String,
        #[arg(long)]
        supply: u64,
    },
    Swap {
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
        #[arg(long)]
        amount: u64,
    },
    Status {
        #[arg(long)]
        symbol: String,
    },
}

#[derive(Subcommand)]
enum DebtCommands {
    IssueBond {
        #[arg(long)]
        symbol: String,
        #[arg(long)]
        principal: u64,
    },
    BuyBond {
        #[arg(long)]
        symbol: String,
        #[arg(long)]
        amount: u64,
    },
    Yield {
        #[arg(long)]
        symbol: String,
    },
}

#[derive(Subcommand)]
enum TradeCommands {
    Long {
        #[arg(long)]
        symbol: String,
        #[arg(long)]
        amount: u64,
        #[arg(long)]
        leverage: f64,
    },
    Short {
        #[arg(long)]
        symbol: String,
        #[arg(long)]
        amount: u64,
        #[arg(long)]
        leverage: f64,
    },
    Liquidate {
        #[arg(long)]
        symbol: String,
        #[arg(long)]
        user: String,
    },
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Commands::Equity { action } => match action {
            EquityCommands::Issue { name, symbol, supply } => {
                let args = equity::actions::IssueArgs { name, symbol, supply };
                equity::actions::issue_equity(args)
            },
            EquityCommands::Swap { from, to, amount } => {
                let args = equity::actions::SwapArgs { from, to, amount };
                equity::actions::swap_equity(args)
            },
            EquityCommands::Status { symbol } => {
                let val = equity::actions::check_status(&symbol);
                val.to_string()
            },
        },
        Commands::Debt { action } => match action {
            DebtCommands::IssueBond { symbol, principal } => {
                debt::bonds::issue_bond(&symbol, principal)
            },
            DebtCommands::BuyBond { symbol, amount } => {
                debt::bonds::buy_bond(&symbol, amount)
            },
            DebtCommands::Yield { symbol } => {
                let val = debt::bonds::check_yield(&symbol);
                val.to_string()
            },
        },
        Commands::Trade { action } => match action {
            TradeCommands::Long { symbol, amount, leverage } => {
                trade::positions::open_position(&symbol, "LONG", amount, leverage)
            },
            TradeCommands::Short { symbol, amount, leverage } => {
                trade::positions::open_position(&symbol, "SHORT", amount, leverage)
            },
            TradeCommands::Liquidate { symbol, user } => {
                trade::positions::liquidate_position(&symbol, &user)
            },
        },
        Commands::Feed { address } => {
            let val = feed::narrative::get_company_narrative(&address);
            val.to_string()
        },
    };

    if cli.json {
        // Ensure result is valid JSON, or wrap it
        if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&result) {
             println!("{}", serde_json::to_string_pretty(&json_val).unwrap());
        } else {
             // If string is not JSON, wrap it
             println!("{}", json!({ "output": result }).to_string());
        }
    } else {
        println!("{}", result);
    }
}
