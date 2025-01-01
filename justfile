

spell:
  typos octizys_*/src/*.rs --color=always | less -r

spellcheck:
  typos octizys_*/src/*.rs

pull:
  git pull origin main

push: spellcheck
  git push origin main

commit: spellcheck
  git commit

install:
  cargo install --path=octizys_repl
