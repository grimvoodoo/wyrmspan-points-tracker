# Wyrmspan Points Tracker

I made this application to make the score totaling at the end of wyrmspan a bit easier, the idea is someone brings this app up on their phone, tablet or laptop at the end of the game and goes though it player by player to total up the scores.

The score for each player will be auto-updated as the values are added. You can either hit the enter or tab keys to move between entries.

Once you have entered the scores for the first player you can click on the `+ add player` button and it will add up to five players.

Once you have entered all the details you can see who has won. Once you have figured out the winner you can click on the `submit` button to add it to the list of high scores.

The top 5 scores will be displayed at the top of the screen for everyone to see.

## How to host this app

I have created a container which is located in the releases of this github repo. You can deploy that with the following command with docker:

```bash
docker run -p 3000:3000 wrymspan-score-tracker:1.0.0
```

or podman

```bash
podman run -p 3000:3000 wrymspan-score-tracker:1.0.0
```

That will start the webserver, which can then be accessed through port 3000 on your [local machine](http://127.0.0.1:3000)

## Not Affiliated With Stonemaier Games

This is a fan-made project for tracking scores in _Wyrmspan_. It’s not an official product and has no association with Stonemaier Games — I’m just a fan who wanted a digital score tracker!

All rights to _Wyrmspan_ and related content belong to Stonemaier Games.

## Future plans

I have used this tracker a few times and have some ideas for how to improve it.

- I want to add the ability to show the results of the most recent game below the high scores list
- I want to add collapseable sections to hide parts like the high scores that people may not care about.
