# ai-screenshot-describer
Just a simple application that receives as input an Area screenshot from Linux and asks for multiple online AI models to describe it.

# Things needed for MVP
1. It does an GET request to LLMs online
    1.1 Reads .env with OPENAI_API_KEY
2. It receives a picture from input
3. Receives the answer 
4. Shows the answer to the user(like alternate text of html) 

# Important Links
https://platform.openai.com/usage
https://platform.openai.com/docs/api-reference/models/list
https://platform.openai.com/docs/api-reference/responses/create