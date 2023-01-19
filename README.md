To test:

* Build
* Copy ./target/debug/trigger-timer somewhere and tar it: `tar czvf trigger-timer.tar.gz trigger-timer`
* Get the SHA and update the manifest: `shasum -a 256 trigger-timer.tar.gz`.  You'll need to update the URL too
* `spin plugin install --file ./trigger-timer.json --yes`

Then you should be able to `spin build --up --follow-all` the guest.
