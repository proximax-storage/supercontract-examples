## Owner only functions
#### ```init()```

Initialize airdrop supercontract

#### ```set_airdrop_divisibility()```

Set airdrop divisibility

Input divisibility in parameter field: ```6``` (example)

#### ```set_participate_fee()```

Set participate fee (gas fee for airdrop transaction)

Input fee in parameter field: ```100``` (example)

#### ```deposit()```

SC creator deposit token to airdrop into the SC

Input mosaic in service payment field: mosaic id and mosaic amount (example)

#### ```prejoin()```

Predefine account to issue airdrop

Input address and amount in parameter field:
```SABNXKV3DLHWCL5IXBQRAUF3SM7HBYFG7SPA2B5T2000``` (example)

address: SABNXKV3DLHWCL5IXBQRAUF3SM7HBYFG7SPA2B5T

amount of airdrop: 2000

#### ```distribute()```

Distribute airdrop based on total participate amount ratio

#### ```autorun.rs```

Autorun.rs is used to determine when to distribute the airdrop.

In this example, this airdrop will release when height is equal to ```20000```

autorun.rs is triggered everytimes new block height is generated. It will check the condition, if condition is met, it will call the ```distribute()```.

How to use:

In the storage user app, input following parameters.

Automatic Execution Number(how many repetition you want it to trigger): 1

Automatic Execution File Name (what file will it call when autorun condition is met): airdrop.wasm

Automatic Execution Function Name (what function in the file will it call): distribute

Automatic Execution Call Payment (fee to run the autorun): 1000 

Automatic Download Call Payment (fee to download file if any): 0 


## User's functions
#### ```join()```

To participate in the airdrop.

Participate_amount = amount-participate_fee

Input address in parameter field: SABNXKV3DLHWCL5IXBQRAUF3SM7HBYFG7SPA2B5T (example)

Input amount of xpx to participate in service payment field: xpx mosaic id and amount

## Utils
#### ```get_airdrop_id()```

Get airdrop mosaic id

#### ```get_airdrop_amount()```

Get total airdrop amount

#### ```get_participate_fee()```

Get participate fee

#### ```get_participant()```

Read airdrop participants address and participate amount

#### ```append_participant()```

Append participant into file for record purposes

#### ```get_divisibility()```

Get token divisibility

#### ```encode_address()```

Convert string address into base32 bytes for sending  transfer transaction purposes
