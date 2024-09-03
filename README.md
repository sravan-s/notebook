# notebook > Revisit apache zepplin :)

Zeppelin is a Web-based notebook that enables data-driven,
interactive data analytics and collaborative documents with SQL, Scala, Python, R and more

Many years ago, I used to work on it.

What if I try to implement a POC of "web-based-notebook"?
1. A tad bit more opinionated
2. Use Firecracker VMs
3. Support a language and DB first; then expand
  * Only JS and create a
4. Redefine the UI
  * Steps to create notebook
  * Setup secrets
5. Implement an API to access/print data
6. Share data among paragraphs using this API
7. How about having a gitlike version control?
  - No autosave, must commit before running
8. Pkg mgmt system for notebook

> A lot of things would be scoped down because, the key part is firecracker
> And almost everything else is a distraction

No ACL/user management for now
No resource specs for now

I develop in Linux, should work in Mac too. Windows probably not
Steps:
* Get devbox https://www.jetify.com/devbox/docs/quickstart/ 
* See `devbox.json` for the requirements(You can manually install those too)
* `devbox shell` to enter shell with all the dev. requirements
* Backend is in `/src`
* Web is in `/web`
* API tests are in `/api_tests`
* You need some dependencies to run Firecracker, for that look at `./linux/README.md`

References:
* https://www.jetify.com/devbox/docs/devbox_examples/languages/rust/
* https://www.nixhub.io/packages
* https://nixos.wiki/wiki/Cleaning_the_nix_store
