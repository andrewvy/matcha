// RPC is going to be a JSON-RPC server (like bitcoin)

// RPC Authentication:
// server boot: creates a `~/.config/matcha/rpc.cookie`
// RPC client uses `rpc.cookie` to authenticate

/* Commands:
 *
 * `getbalance <address_name>` - If `address_name` not provided, return total balance of all addresses.
 * `getnewaddress <address_name>` - `address_name` required, creates a new address as `address_name`.
 * `listwallet` - Returns wallet containing all addresses and amounts.
 *
 * -- Transactions
 * `send <from_address_name> <address> <amount>` - Sends `amount` from `from_address_name` to the given `address`.
 * `sendvote <delegate_address> <amount>` - Sends a vote for `delegate_address`, using `amount`.
 * `registerusername <from_address_name> <username>` - Registers an address to a `username`.
 * `newpost <from_address_name> <content>` - Creates a new post under a given address.
 * `favoritepost <txid> <txout_index>` - Favorites a post.
 */
