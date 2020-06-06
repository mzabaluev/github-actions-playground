# Two-Stage Release Workflow

The workflow in this branch tests creation of releases in two stages.
First a draft is created to get the upload URL, to which the assets
are uploaded. Then the release should be published by removing the draft
attribute, or so the theory goes. The notification email, when sent on
published release, should list both the sources and the added asset.
