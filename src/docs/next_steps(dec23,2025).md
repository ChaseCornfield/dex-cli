Goal decomposition (minimal steps)
Step 1: Stop printing raw JSON; parse into typed structs
You already know channel is "bbo" or "trades". Next is: define Rust structs/enums that match each payload and deserialize.
Resource: `serde` derive + deserialization, `serde_json` basics

Step 2: Maintain “UI state” in memory
Keep:
latest BBO (best bid + best ask)
recent trades (e.g., last 50 in a VecDeque)
Topic to learn: ring buffer / VecDeque
Resource:
Step 3: Split the program into producer/consumer
One async task reads the websocket stream and sends parsed updates to a channel.
Another task renders the CLI using the latest state.
Topic to learn: async channels + task spawning
Resources: ,
Step 4: Render loop + input
Start simple: redraw every ~100ms (“tick”) and show a table: last trade, mid price, bid/ask, spread.
Topic to learn: coordinating “tick”, “input”, and “incoming data”
Resource:
