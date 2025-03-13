## The airdrop contract manages the distribution of tokens (such as XPX) to participants based on their contribution ratio (between certain block heights). 

### Airdrop calculation

  reward = X/Y*Z

  X= your lowest balance between block y and block z

  Y= sum total of all participants' lowest balance between block y and block z.

  Z= total airdrop

**How to use:**

Contract creator
1. Contract creator calls init() to initialize the airdop contract.
2. Contract creator calls set_end_height() to determine the airdrop blockheight.
3. Contract creator calls set_participate_fee() to determine the users' airdrop participation fee.
4. Contract creator calls deposit() to deposit token to airdrop.

Users
1. Users call the join() to participate the airdrop event.

Supercontract
1. When the airdrop height is reached, the airdrop will be automatically distributed to participants based on the airdrop calculation.

### Owner's functions

#### ```init()```

    Initialize airdrop supercontract

#### ```set_airdrop_divisibility()```

    Set airdrop divisibility

    Input divisibility in parameter field: 6 (example)

#### ```set_participate_fee()```

    Set participate fee (gas fee for airdrop transaction)

    Input fee in parameter field: 100 (example)

#### ```set_end_height()```

    Set last block height to join the airdrop

    Input height in parameter field: 10 000 (example)

#### ```deposit()```

    SC creator deposit token to airdrop into the SC

    Input mosaic in service payment field: mosaic id and mosaic amount (example)

#### ```prejoin()```

    Predefine account to issue airdrop

    Input address and amount in parameter field:
    SABNXKV3DLHWCL5IXBQRAUF3SM7HBYFG7SPA2B5T2000 (example)

    address: SABNXKV3DLHWCL5IXBQRAUF3SM7HBYFG7SPA2B5T

    amount of airdrop: 2000

#### ```distribute()```

    Distribute airdrop based on participate amount ratio

    *** Operation require execution payment and download payment

#### ```autorun.rs```

    Autorun.rs is used to determine when to distribute the airdrop.

    In this example, this airdrop will release when height is equal to 20000

    autorun.rs is triggered everytimes new block height is generated. It will check the condition, if condition is met, it will call the ```distribute()```.


### User's functions
#### ```join()```

    To participate the airdrop.

    Participate_amount = amount-participate_fee

    Input address in parameter field: SABNXKV3DLHWCL5IXBQRAUF3SM7HBYFG7SPA2B5T (example)

    Input participate fee in service payment field: xpx mosaic id and amount

    *** Operation require execution payment and download payment

## Utils
#### ```get_airdrop_id()```

    Get airdrop mosaic id

#### ```get_airdrop_amount()```

    Get total airdrop amount

#### ```set_airdrop_amount()```

    Set total airdrop amount

#### ```get_participate_fee()```

    Get participate fee

#### ```get_end_height()```

    Get last height to join the airdrop

#### ```get_participant()```

    Read airdrop participants address and participate amount

#### ```append_participant()```

    Append participant into file for record purposes

#### ```get_divisibility()```

    Get token divisibility

#### ```get_prejoin_count()```

    Read get prejoin counter for airdrop purpose

#### ```set_prejoin_count()```

    Set prejoin counter for airdrop purpose

#### ```get_participant_count()```

    Read get participant counter for airdrop purpose

#### ```set_participant_count()```

    Set participant counter for airdrop purpose

#### ```encode_address()```

    Convert string address into base32 bytes for sending  transfer transaction purposes

#### ```decode_mosaic()```

    Decode mosaic id and amount from rest endpoint format to u64 format
