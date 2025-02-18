### This Supercontract implements a simple exchange functionality.  Sellers deposit their asset into the contract along with the desired price (specified in a particular token and amount). Buyers can then purchase the asset by sending the correct token ID and amount to the contract. Upon successful matching of the seller's price and the buyer's offer, the Supercontract automatically transfers the asset to the buyer and the specified tokens to the seller.

#### ```deposit_asset()```
This function is used during contract deployment. The contract caller must specify the asset they wish to sell as payment for the service, along with the amount they intend to receive in the call parameters.

#### ```buy_asset()```
This function facilitates asset purchase. The contract caller is required to provide the mosaic ID and amount as service payment, along with the caller's address in the call parameters.
