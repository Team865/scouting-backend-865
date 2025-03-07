## Adding new games

note: Name means the name of the game in Pascal case, YYYY means the year of the game (i.e. Name = Reefscape, YYYY = 2025)

1. add a file to `src/data` named `game_YYYY.rs`
2. add structs that match your data (see [`src/data/game_2025.rs`](src/games/game_2025.rs) for an example),
   and can be (de)serialized with serde. also make sure the main one is called `GameDataYYYY` and implements `Into<Vec<String>>`.
3. add `NameYYYY(GameDataYYYY)` into `GameSpecificData` in `src/data.rs`
4. add `GameSpecificData::NameYYYY(data) => fields.append(&mut data.into())` to `GameData`'s `Into<Vec<String>>::into` in `src/data.rs`

## Setting up your own version

1. [set up the frontend](https://github.com/Team865/scouting-app-865)
2. set up a Google Cloud Console project, and make a service account that can access your spreadsheet
3. make a file called `settings.json`:
```json
{
  "root": "/warp7api/scouting",
  "address": "127.0.0.1",
  "port": 42069,
  "frontend": "*",
  "credentials_path": "service_account.json",
  "spreadsheet_id": "<your spreadsheet ID>",
  "main_worksheet": "Raw Data",
  "test_worksheet": "Test Data"
}
```
4. run the backend and send a report to it from your frontend to make sure it works
5. set up nginx or something to proxy the frontend and backend onto the same domain, and secure everything.
   (this step is, although just generally good advice, mainly to prevent friction from CORS)
