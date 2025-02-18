### This Supercontract allows up to 20 players to join each game by purchasing a ticket for 100 XPX. Once 20 players have joined, a winner is randomly selected. The winner receives 95% of the total tokens, while the remaining 5% is retained in the Supercontract account.

#### ```init()```
This function is called when deploying the contract. It initializes the necessary files to run the contract.

#### ```join()```
This function allows players to join the lottery. The contract caller must provide 100 XPX in the service payment field and input their account address in the call parameter.

#### ```distribute()```
This function distributes the prize to the winner. It is designed to be triggered by the autorun feature, which automatically sends the prize or refunds the ticket amount if fewer than 20 players join the game.
