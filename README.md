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

No ACL/user management for now
No resource specs for now

I develop in Linux, should work in Mac too. Windows probably not
Steps:
* Get https://www.jetify.com/devbox/docs/quickstart/ 
* See `devbox.json` for the requirements(You can manually install those too)
* `devbox shell` to enter shell with all the dev. requirements
