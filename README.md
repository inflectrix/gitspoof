# gitspoof
A demonstrational tool to get people to appear in your contributors list.

### Disclaimer
*This repo was made as a joke and example and is not meant to be used in any scandals*

### How it works
Once installed, you can do `gitspoof --help` to see the args. 
`--repo` is the path to the repository, `--name` is the display name of the person you want to spoof, and 
`--email` is the email of the person you are trying to spoof - **GitHub uses the email when checking commits against accounts, so this is the most important parameter**, and 
`--message` is the commit name/message.

When you push the commit, it should have the person whose email you specified as the author of the commit, GitHub account linked and all.
Due to unknown reasons, certain accounts may take longer than others to appear on the contribution graph.
