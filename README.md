## SOLANA SPL TOKEN PRESALE PROGRAM ##
![Screenshot (177)](https://github.com/user-attachments/assets/7be3af85-0999-4571-922d-b12ccc82a154)

<a href="https://firebasestorage.googleapis.com/v0/b/ashar-2023.appspot.com/o/screen_record.mp4?alt=media" > See Demo </a>

### Contract Overview:

This contract, written using the Anchor framework on Solana is a token presale. Here's what it does:

1. **create_presale**: This function sets up the parameters for a presale, such as:
   - **token_mint_address**: The token being sold.
   - **softcap_amount**: The minimum amount of funds to be raised for the presale to succeed.
   - **hardcap_amount**: The maximum amount of funds the presale can raise.
   - **max_token_amount_per_address**: Limit on how many tokens an individual can purchase.
   - **price_per_token**: The price per token.
   - **start_time** and **end_time**: Time window for the presale.

2. **update_presale**: This function allows updating presale parameters, such as the token price or time limits, if needed.

3. **deposit_token**: This likely allows the presale manager or owner to deposit the tokens that will be sold during the presale.

4. **start_presale**: Initiates the presale by setting the start and end times.

5. **buy_token**: Enables users to purchase tokens using a "quote token" (often SOL or another stable token) by specifying the amount of tokens they want to buy and the corresponding payment.

6. **claim_token**: After the presale is over, buyers can claim their purchased tokens.

7. **withdraw_sol**: The contract owner can withdraw the SOL (or other quote tokens) collected during the presale after it ends.

8. **withdraw_token**: If the presale is unsuccessful or canceled, the owner may be able to withdraw the tokens that were not sold.

### Real-World Use Case:

This kind of contract would be used in **token fundraising**. Projects can launch presales to secure early capital and reward early supporters by offering them the project’s tokens at discounted rates. Real-world scenarios might include:
- **Blockchain startups** raising funds to develop their platform.
- **NFT platforms** releasing tokens to attract early users.
- **Gaming or DeFi projects** issuing governance or utility tokens pre-launch.

This contract helps manage the sale of these tokens, ensuring limits, security, and proper fund collection during the process.
