# wg-plumber

## Commands

* `show interface brief`

* `show interface <INTERFACE>`

* `show interface all`

* `show interface <INTERFACE>|all pubkey`

* `show interface <INTERFACE>|all privkey`

* `show interface <INTERFACE>|all preshared-keys`

* `show interface <INTERFACE>|all endpoints`

* `show interface <INTERFACE>|all allowed-ips`

* `show interface <INTERFACE>|all handshakes`

* `show interface <INTERFACE>|all transfers`

* `show interface <INTERFACE>|all keepalive`

* `show interface <INTERFACE>|all listen-port`

* `show interface <INTERFACE>|all dump`

* `delete interface <INTERFACE> peer <PEER_PUB_KEY>`

* `set interface <INTERFACE>|all up`

* `set interface <INTERFACE>|all down`

* `set interface <INTERFACE> listen-port <LISTEN_PORT>`

* `set interface <INTERFACE> private-key [<PRIVATE_KEY>]`

* `set interface <INTERFACE> peer [<PEER_PUB_KEY>] private-key [<PEER_PRIV_KEY>]`

* `set interface <INTERFACE> peer <PEER_PUB_KEY> allowed-ips <ALLOWED_IP_1>[...<ALLOWED_IP_N>]`

* `set interface <INTERFACE> peer <PEER_PUB_KEY> endpoint <ENDPOINT_ADDRESS>:<ENDPOINT_PORT>`

* `set interface <INTERFACE> peer <PEER_PUB_KEY> preshared-key <PRESHARED-KEY>`

* `config save <OUT_PATH>`

* `config load <IN_PATH>`

* `config reload`



## Use Case Procedures

* Create Interface
  * `set interface <INTERFACE>`
  * `set interface <INTERFACE> private-key`
  * `set interface <INTERFACE> listen-port <PORT>`
* Modify Interface
  * change listen port
  * change private key
    * re-generate profiles
* Delete Interface
  * `delete interface <INTERFACE>`
* Add Peer to Interface
  * `set interface <INTERFACE> peer new`
  * `set interface <INTERFACE> peer <PEER_PUB_KEY> allowed-ips <ALLOWED_IPS>`
  * `set interface <INTERFACE> peer <PEER_PUB_KEY> pub-key <PUBLIC_KEY>`
  * `set interface <INTERFACE> peer <PEER_PUB_KEY> endpoint <ADDRESS>:<PORT>`
  * `set interface <INTERFACE> peer <PEER_PUB_KEY> preshared-key <PRESHARED_KEY>`
  * `set interface <INTERFACE> peer <PEER_PUB_KEY> keepalive <KEEPALIVE>`
* Modify Interface Peer
  * change allowed-ips
    * `set interface <INTERFACE> peer <PEER_PUB_KEY> allowed-ips <ALLOWED_IPS>`
  * change public key
    * `set interface <INTERFACE> peer <PEER_PUB_KEY> public-key <PUBLIC_KEY>`
  * change private key
    * `set interface <INTERFACE> peer <PEER_PUB_KEY> private-key <PRIVATE_KEY>`
  * change endpoint
    * `set interface <INTERFACE> peer <PEER_PUB_KEY> endpoint <ADDRESS:PORT>`
  * change preshared-key
    * `set interface <INTERFACE> peer <PEER_PUB_KEY> preshared-key <PRESHARED_KEY>`
  * change keepalive
    * `set interface <INTERFACE> peer <PEER_PUB_KEY> keepalive <KEEPALIVE>`
* Delete Interface Peer
  * `delete interface <INTERFACE> peer <PEER_PUB_KEY>`
* Show Interface Information
  * list interfaces
    * `show interface brief`
  * show full data on all interfaces
    * `show interface all`
  * list public keys for all interfaces
    * `show interface all public-key`
  * list private keys for all interfaces
    * `show interface all private-key`
  * list preshared keys for all interfaces
    * `show interface all preshared-keys`
  * list endpoints for all interfaces
    * `show interface all endpoints`
  * list allowed-ips for all interfaces
    * `show interface all allowed-ips`
  * list handshakes for all interfaces
    * `show interface all handshakes`
  * list transfers for all interfaces
    * `show interface all transfers`
  * list keepalive for all interfaces
    * `show interface all keepalive`
  * list listen-ports for all interfaces
    * `show interface all listen-port`
  * dump info for all interfaces
    * `show interface all dump`
  * show full data on an interfaces
    * `show interface <INTERFACE>`
  * list public keys for interface
    * `show interface <INTERFACE> public-key`
  * list private keys for interface
    * `show interface <INTERFACE> private-key`
  * list preshared keys for interface
    * `show interface <INTERFACE> preshared-keys`
  * list endpoints for interface
    * `show interface <INTERFACE> endpoints`
  * list allowed IPs for interface
    * `show interface <INTERFACE> <INTERFACE>owed-ips`
  * list handshakes for interface
    * `show interface <INTERFACE> handshakes`
  * list transfers for interface
    * `show interface <INTERFACE> transfers`
  * list keepalive for interface
    * `show interface <INTERFACE> keepalive`
  * list listen-ports for interface
    * `show interface <INTERFACE> listen-port`
  * dump info for interface
    * `show interface <INTERFACE> dump`
* Bring interface up
  * `set interface <INTERFACE> up`
* Bring interface down
  * `set interface <INTERFACE> down`
* Bring all interfaces up
  * `set interface all up`
* Bring all interfaces down
  * `set interface all down`
* Load configuration file, deleting existing interfaces, and replacing them with those in the file
  * `config load <CONFIG_FILE>`
* Save configure file to a new file, overwriting if the path exists and creating a new file otherwise
  * `config save <CONFIG_FILE>`
* Reload configuration file from disk
  * `config reload`