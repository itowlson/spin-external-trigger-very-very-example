To test:

* Build
* Copy ./target/debug/timer somewhere and tar it: `tar czvf timer.tar.gz timer`
* Get the SHA and update the manifest: `shasum -a 256 timer.tar.gz`.  You'll need to update the URL too
* `spin plugin install --file ./timer.json --yes`

Then you should be able to `spin build --up` the guest.
