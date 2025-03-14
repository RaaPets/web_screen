BIND = '127.0.0.2'
PORT = 8188

help:
	@cat Makefile

run:
	@cargo run -r -- -b $(BIND) -p $(PORT)
winrun:
	@cargo run -r --target=x86_64-pc-windows-gnu -- -b $(BIND) -p $(PORT)

# # # # # # # #
pull:
	@git pull

savetogit: git.pushall
git.pushall: git.commitall
	@git push
git.commitall: git.addall
	@if [ -n "$(shell git status -s)" ] ; then git commit -m 'saving'; else echo '--- nothing to commit'; fi
git.addall:
	@git add .

# # # # # # # #
clean:
	@cargo clean
