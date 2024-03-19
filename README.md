# Tauri Redis Desktop Manager

> A Redis Desktop Manager powered by Rust、Tauri、Vite、Typescript、React、Mobx、ant-design、tailwindcss with beautiful UI

## Feature

+ Dark Mode
+ Cluster Mode
+ Readonly Mode
+ SSH Proxy
+ Scan、Sscan、Hscan、Zscan For Key/Field Paginate
+ Publish/Subscribe
+ Monitor
+ Key Memory Analysis  
+ Multiple Window
+ Debug Log
+ Data Migrate
+ JSON Type
+ Probabilistic data structures(TopK,HyperLogLog,Bloom filter,Count-min sketch,t-digest)
+ Time Series structures
+ Terminal
  
## Preview

UI
![image](https://github.com/zjwshisb/tauri-redis-desktop-manager/blob/main/screen/preview-1.png?raw=true)

Dark Mode
![image](https://github.com/zjwshisb/tauri-redis-desktop-manager/blob/main/screen/preview-2.png?raw=true)

Memory Analysis
![image](https://github.com/zjwshisb/tauri-redis-desktop-manager/blob/main/screen/memory.png?raw=true)

Migrate
![image](https://github.com/zjwshisb/tauri-redis-desktop-manager/blob/main/screen/migrate.png?raw=true)
  
## Command Support (In GUI)

Those command which be checked are supported  at  GUI

#### generic

- [X] COPY
- [X] DEL
- [X] DUMP
- [ ] EXISTS
- [X] EXPIRE
- [X] EXPIREAT
- [X] EXPIRETIME
- [ ] KEYS
- [ ] MIGRATE
- [X] MOVE
- [X] OBJECT ENCODING
- [X] OBJECT FREQ
- [X] OBJECT IDLETIME
- [X] OBJECT REFCOUNT
- [X] PERSIST
- [X] PEXPIRE
- [X] PEXPIREAT
- [X] PEXPIRETIME
- [X] PTTL
- [X] RANDOMKEY
- [X] RENAME
- [X] RENAMENX
- [X] RESOTRE
- [X] SCAN
- [ ] SORT
- [ ] SORT_RO
- [ ] TOUCH
- [X] TTL
- [X] TYPE
- [X] UNLINK
- [ ] WAIT
- [ ] WAITAOF

#### STRING

- [x] APPEND
- [X] DECR
- [X] DECRBY
- [X] GET
- [X] GETDEL
- [X] GETRANGE
- [X] GETSET
- [X] INCR
- [X] INCRBY
- [X] INCRBYFLOAT
- [X] LCS
- [X] MGET
- [X] MSET
- [ ] ~~MSETNX~~(deprecated)
- [ ] ~~PSETEX~~(deprecated)
- [X] SET
- [ ] ~~SETEX~~(deprecated)
- [ ] ~~SETNX~~(deprecated)
- [X] SETRANGE
- [X] STRLEN
- [ ] ~~SUBSTR~~(deprecated)

#### SET

- [x] SADD
- [X] SCARD
- [X] SDIFF
- [X] SDIFFSTORE
- [X] SINTER
- [X] SINTERCARD
- [X] SINTERSTORE
- [X] SISMEMBER
- [X] SMEMBERS
- [X] SMISMEMBER
- [X] SMOVE
- [X] SPOP
- [X] SRANDMEMBER
- [X] SREM
- [X] SSCAN
- [X] SUNION
- [X] SUNIONSTORE

#### LIST

- [X] BLMOVE
- [X] BLMPOP
- [X] BLPOP
- [X] BRPOP
- [X] BRPOPLPUSH
- [X] LINDEX
- [X] LINSERT
- [X] LLEN
- [X] LMOVE
- [X] LMPOP
- [X] LOP
- [X] LPOS
- [X] LPUSH
- [X] LPUSHX
- [X] LRANGE
- [X] LREM
- [X] LSET
- [X] LTRIM
- [X] RPOP
- [X] RPOPLPUSH
- [X] RPUSH
- [X] RPUSHX

#### Hash

- [X] HEL
- [X] HEXISTS
- [X] HGET
- [X] HGETALL
- [X] HINCRBY
- [X] HINCRBYFLOAT
- [X] HKEYS
- [X] HLEN
- [X] HMGET
- [ ] ~~HMSET~~(deprecated)
- [X] HRANDFIELD
- [X] HSCAN
- [X] HSET
- [X] HSETNX
- [X] HSTRLEN
- [X] HVALS

#### Sorted Set

- [X] BZMPOP
- [X] BZPOPMAX
- [X] BZPOPMIN
- [X] ZADD
- [X] ZCARD
- [X] ZCOUNT
- [X] ZDIFF
- [X] ZDIFFSTORE
- [X] ZINCRBY
- [X] ZINTER
- [X] ZINTERCARD
- [X] ZINTERSTORE
- [X] ZLEXCOUNT
- [X] ZMPOP
- [X] ZMSCORE
- [X] ZPOPMAX
- [X] ZPOPMIN
- [X] ZRANDMEMBER
- [X] ZRANGE
- [ ] ZRANGEBYLEX
- [ ] ZRANGEBYSCORE
- [X] ZRANGESTORE
- [X] ZRANK
- [X] ZREM
- [X] ZREMRANGEBYLEX
- [X] ZREMRANGEBYSCORE
- [ ] ZREVRANGE
- [ ] ZREVRANGEBYLEX
- [ ] ZREVRANGEBYSCORE
- [X] ZREVRANK
- [X] ZSCAN
- [X] ZSCORE
- [X] ZUNION
- [X] ZUNIONSOTRE

#### Pub/Sub

- [ ] PSUBSCRIBE
- [X] PUBLISH
- [ ] PUBSUB CHANNELS
- [ ] PUBSUB NUMPAT
- [ ] PUBSUB NUMSUB
- [ ] PUBSUB SHARDCHANNELS
- [ ] PUBSUB SHARDNUMSUB
- [ ] PUBSUBSCRIBE
- [ ] SPUBLISH
- [ ] SSUBSCRIBE
- [x] SUBSCRIBE
- [ ] SUBSUBSCRIBE
- [ ] UNSUBSCRIBE

#### HyperLogLog

- [X] PFADD
- [X] PFCOUNT
- [ ] PFDEBUG
- [ ] PFMERGE
- [ ] PFSELFTEST

#### JSON

- [ ] JSON.ARRAPPEND
- [ ] JSON.ARRINDEX
- [ ] JSON.ARRINSERT
- [ ] JSON.ARRLEN
- [ ] JSON.ARRPOP
- [ ] JSON.ARRTRIM
- [ ] JSON.ARRCLEAR
- [ ] JSON.CLEAR
- [ ] JSON.DEBUG
- [X] JSON.DEBUG MEMORY
- [ ] JSON.DEL
- [ ] JSON.FORGET
- [X] JSON.GET
- [ ] JSON.MERGE
- [ ] JSON.MGET
- [ ] JSON.MSET
- [ ] JSON.NUMINCRBY
- [ ] JSON.NUMMULTBY
- [ ] JSON.OBJKEYS
- [ ] JSON.OBJLEN
- [ ] JSON.RESP
- [X] JSON.SET
- [ ] JSON.STRARRPEND
- [ ] JSON.STRLEN
- [ ] JSON.TOGGLE
- [ ] JSON.TYPE

#### Bloom Filter

- [ ] BF.ADD
- [X] BF.CARD
- [X] BF.EXISTS
- [X] BF.INFO
- [ ] BF.INSERT
- [ ] BF.LOADCHUNK
- [X] BF.MADD
- [ ] BF.MEXISTS
- [X] BF.RESERVE
- [ ] BF.SCANDUMP

#### Cuckoo Filter

- [X] CF.ADD
- [X] CF.ADDNX
- [X] CF.COUNT
- [X] CF.DEL
- [X] CF.EXISTS
- [X] CF.INFO
- [X] CF.INSERT
- [X] CF.INSERTNX
- [ ] CF.LOADCHUNK
- [X] CF.MEXISTS
- [X] CF.RESERVE
- [X] CF.SCANDUMP

#### T-digest

- [x] TDIGEST.ADD
- [x] TDIGEST.BYRANK
- [x] TDIGEST.BYREVRANK
- [x] TDIGEST.CDF
- [x] TDIGEST.CREATE
- [x] TDIGEST.INFO
- [x] TDIGEST.MAX
- [ ] TDIGEST.MERGE
- [x] TDIGEST.MIN
- [x] TDIGEST.QUANTILE
- [x] TDIGEST.RANK
- [x] TDIGEST.RESET
- [x] TDIGEST.REVRANK
- [x] TDIGEST.TRIMMED_MEAN

#### Time Series

- [X] TS.ADD
- [X] TS.ALTER
- [X] TS.CREATE
- [X] TS.CREATERULE
- [X] TS.DECRBY
- [X] TS.DEL
- [X] TS.DELETERULE
- [ ] TS.GET
- [X] TS.INCRBY
- [X] TS.INFO
- [ ] TS.MADD
- [ ] TS.MGET
- [ ] TS.MRANGE
- [ ] TS.MREVERANGE
- [ ] TS.QUERYINDEX
- [X] TS.RANGE
- [ ] TS.REVERANGE

#### Top-k

- [X] TOPK.ADD
- [X] TOPK.COUNT
- [X] TOPK.INCRBY
- [X] TOPK.INFO
- [X] TOPK.LIST
- [X] TOPK.QUERY
- [X] TOPK.RESERVE

#### Count-Min Sketch

- [X] CMS.INCRBY
- [X] CMS.INFO
- [X] CMS.INITBYDIM
- [X] CMS.INITBYPROB
- [X] CMS.MERGE
- [X] CMD.QUERY

#### Connection Management

- [X] AUTH
- [ ] CLIENT CACHING
- [ ] CLIENT GETNAME
- [ ] CLIENT GETREDIR
- [ ] CLIENT ID
- [ ] CLIENT INFO
- [X] CLIENT KILL
- [X] CLIENT LIST
- [ ] CLIENT NO-EVICT
- [ ] CLIENT NO-TOUCH
- [ ] CLIENT PAUSE
- [ ] CLIENT REPLY
- [ ] CLIENT SETINFO
- [ ] CLIENT SETNAME
- [ ] CLIENT TRACKING
- [ ] CLIENT TRACKINGINFO
- [ ] CLIENT UNBLOCK
- [ ] CLIENT UNPAUSE
- [ ] ECHO
- [ ] HELLO
- [X] PING
- [ ] QUIT
- [ ] RESET
- [X] SELECT

#### Cluster Management

- [ ] ASKING
- [ ] CLUSTER ADDSLOTS
- [ ] CLUSTER ADDSLOTSRANGE
- [ ] CLUSTER BUMPEPOCH
- [ ] CLUSTER COUNT-FAILURE-REPORTS
- [ ] CLUSTER COUNTKEYSINSLOT
- [ ] CLUSTER DELSLOTS
- [ ] CLUSTER DELSLOTSRANGE
- [ ] CLUSTER FAILOVER
- [ ] CLUSTER FLUSHSLOTS
- [ ] CLUSTER FORGET
- [ ] CLUSTER GETKEYSINSLOT
- [ ] CLUSTER INFO
- [ ] CLUSTER KEYSLOT
- [ ] CLUSTER LINKS
- [ ] CLUSTER MEET
- [ ] CLUSTER MYID
- [ ] CLUSTER MYSHARDID
- [X] CLUSTER NODES
- [ ] CLUSTER REPLICAS
- [ ] CLUSTER REPLICATE
- [ ] CLUSTER RESET
- [ ] CLUSTER SAVECONFIG
- [ ] CLUSTER SET-CONFIG-EPOCH
- [ ] CLUSTER SETSLOT
- [ ] CLUSTER SHARDS
- [ ] CLUSTER SLAVES
- [ ] CLUSTER SLOTS
- [ ] READONLY
- [ ] READWRITE

#### Server Management

- [ ] ACL CAT
- [ ] ACL DELUSER
- [ ] ACL DRYRUN
- [ ] ACL GENPASS
- [ ] ACL GETUSER
- [ ] ACL LIST
- [ ] ACL LOAD
- [ ] ACL LOG
- [ ] ACL SAVE
- [ ] ACL SETUSER
- [ ] ACL USERS
- [ ] ACL WHOAMI
- [ ] BGREWRITEAOF
- [ ] BGSAVE
- [ ] COMMAND
- [ ] COMMAND COUNT
- [ ] COMMAND DOCS
- [ ] COMMAND GETKEYS
- [ ] COMMAND GETKEYSANDFLAGS
- [ ] COMMAND INFO
- [ ] COMMAND LIST
- [X] CONFIG GET
- [X] CONFIG RESETSTAT
- [X] CONFIG REWRITE
- [X] CONFIG SET
- [X] DBSIZE
- [ ] FAILOVER
- [ ] FLUSHALL
- [X] FLUSHDB
- [X] INFO
- [ ] LASTSAVE
- [ ] LANTENCY DOCTOR
- [ ] LANTENCY GRAPH
- [ ] LANTENCY HISTOGRAM
- [ ] LANTENCY HISTORY
- [ ] LANTENCY LATEST
- [ ] LANTENCY RESET
- [ ] LOLWUT
- [X] MEMORY DOCTOR
- [ ] MEMORY MALLOC-STATS
- [ ] MEMORY PURGE
- [X] MEMORY STATS
- [X] MEMORY USAGE
- [X] MODULE LIST
- [ ] MODULE LOAD
- [ ] MODULE LOADEX
- [ ] MODULE UNLOAD
- [X] MONITOR
- [ ] PSYNC
- [ ] REPLCONF
- [ ] REPLICAOF
- [ ] RESTORE-ASKING
- [ ] ROLE
- [ ] SAVE
- [ ] SHUTDOWN
- [ ] SLAVEOF
- [X] SLOWLOG GET
- [X] SLOWLOG LEN
- [X] SLOWLOG RESET
- [ ] SWAPDB
- [ ] SYNC
- [ ] TIME



## Development

+ Backend [Tauri](https://tauri.app/v1/guides/development/development-cycle/)
+ Frontend [Vite](https://vitejs.dev/)

This Project is developed on MacOS and only testing in MacOS.
If you has any problem about this project,It is very welcome to make a issue.
If you are interested in this project, It is  very welcome make a PR.

## LICENSE

MIT
