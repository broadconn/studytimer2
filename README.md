# Study App
In a world full of distraction, we need all the help we can get to stay focused. 

This app is a visual reminder on your screen to keep studying for the duration of your defined study session. Add your study topics to a list along with the length of time you wish to study each one, and hit the play button. Includes a setting for break duration.

![pic](https://i.imgur.com/8ctoEZf.png)

It additionally provides a space to let you save and organize your intended future study topics, so they're never out of mind and can be quickly added to your current study session if desired.

![pic](https://i.imgur.com/eKIUUR9.png)

## Developing
This is a Svelte app rendered within Tauri. Styled with Tailwind.

Install dependencies with `yarn install` (or `npm install` or `pnpm install`).

### Start a development server


```bash
yarn start-tauri 
```


## Building

```bash
npm run tauri build
```



# Code Guide
## /src-tauri/src/main.rs
This is the entry point for the Tauri code, and it is where we customize the window and the menu bar. It is also where we define the event listeners for controlling / adjusting the Tauri window via Svelte.

## /src/routes/+page.svelte
The root page displayed by the Svelte app.

## Port
The port the app uses is defined in two places: `tauri.conf.json` and `vite.config.ts`. Both files must have the same value.

# Icon
Replace the app-icon.png in the root directory with your new icon, then run
`npm run tauri icon`

# Todo
- Reduce frequency that the visualizer updates if the timer is running for a longer time period
- Improve the visual association between adding a study item and the timer.
  - This is mainly a problem when the studylist is visible - I think largely due to the resulting distance between the timer and the study item you click on.
  - Potential solutions: 
    - Move the timer under the studylist when it's visible
    - Remove the global timer when the list is visible and add smaller timers to each list item that are visible on hover.
      - This has the added benefit of being able to save a custom time per study item. There are a few study items that I always set to a smaller time, e.g. for warm-up exercises. It would be nice to not have to manually adjust the timer for those each time, as the current experience requires.
      - Consider changing the delete-study-item interaction if this solution is chosen. The studyitem UI will get crowded.
- Experiment with making the UI changes centered around the Play button area, considering that's where the buttons are that trigger the changes. It feels jarring to have them move so far from the mouse.