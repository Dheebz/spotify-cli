_spotify_cli_completion() {
  local cur prev words cword

  if type _init_completion >/dev/null 2>&1; then
    _init_completion || return
  else
    words=("${COMP_WORDS[@]}")
    cword=$COMP_CWORD
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
  fi

  local global_flags="--json -v --no-trunc --width"

  if [[ "$cur" == -* ]]; then
    COMPREPLY=( $(compgen -W "$global_flags" -- "$cur") )
    return
  fi

  if [[ $cword -eq 1 ]]; then
    COMPREPLY=( $(compgen -W "auth player track search album artist playlist device help system" -- "$cur") )
    return
  fi

  local cmd1="${words[1]}"
  case "$cmd1" in
    auth)
      if [[ $cword -eq 2 ]]; then
        COMPREPLY=( $(compgen -W "login status scopes logout" -- "$cur") )
        return
      fi
      if [[ "$cur" == -* ]]; then
        COMPREPLY=( $(compgen -W "--client-id" -- "$cur") )
        return
      fi
      ;;
    player)
      if [[ $cword -eq 2 ]]; then
        COMPREPLY=( $(compgen -W "play pause next prev status" -- "$cur") )
        return
      fi
      ;;
    track)
      if [[ $cword -eq 2 ]]; then
        COMPREPLY=( $(compgen -W "like unlike info" -- "$cur") )
        return
      fi
      ;;
    search)
      if [[ "$cur" == -* ]]; then
        COMPREPLY=( $(compgen -W "--type --fuzzy --user --limit --pick --last --play" -- "$cur") )
        return
      fi
      ;;
    album|artist)
      if [[ $cword -eq 2 ]]; then
        COMPREPLY=( $(compgen -W "info play" -- "$cur") )
        return
      fi
      if [[ "$cur" == -* ]]; then
        COMPREPLY=( $(compgen -W "--fuzzy --user --pick --last --search" -- "$cur") )
        return
      fi
      ;;
    playlist)
      if [[ $cword -eq 2 ]]; then
        COMPREPLY=( $(compgen -W "list info play add follow unfollow pin" -- "$cur") )
        return
      fi
      if [[ "$prev" == "--sort" ]]; then
        COMPREPLY=( $(compgen -W "name owner public collaborative" -- "$cur") )
        return
      fi
      case "${words[2]}" in
        list)
          if [[ "$cur" == -* ]]; then
            COMPREPLY=( $(compgen -W "--collaborative --owned --public --private --sort" -- "$cur") )
            return
          fi
          ;;
        info|play|add|follow|unfollow)
          if [[ "$cur" == -* ]]; then
            COMPREPLY=( $(compgen -W "--fuzzy --user --pick --last" -- "$cur") )
            return
          fi
          local IFS=$'\n'
          COMPREPLY=( $(compgen -W "$(spotify-cli complete playlist 2>/dev/null)" -- "$cur") )
          return
          ;;
        pin)
          if [[ $cword -eq 3 ]]; then
            COMPREPLY=( $(compgen -W "add remove list play" -- "$cur") )
            return
          fi
          case "${words[3]}" in
            remove|play)
              local IFS=$'\n'
              COMPREPLY=( $(compgen -W "$(spotify-cli complete pin 2>/dev/null)" -- "$cur") )
              return
              ;;
          esac
          ;;
      esac
      ;;
    device)
      if [[ $cword -eq 2 ]]; then
        COMPREPLY=( $(compgen -W "list set" -- "$cur") )
        return
      fi
      if [[ "${words[2]}" == "list" && "$cur" == -* ]]; then
        COMPREPLY=( $(compgen -W "--live" -- "$cur") )
        return
      fi
      if [[ "${words[2]}" == "set" ]]; then
        local IFS=$'\n'
        COMPREPLY=( $(compgen -W "$(spotify-cli complete device 2>/dev/null)" -- "$cur") )
        return
      fi
      ;;
    completions)
      if [[ $cword -eq 2 ]]; then
        COMPREPLY=( $(compgen -W "bash zsh fish" -- "$cur") )
        return
      fi
      ;;
    system)
      if [[ $cword -eq 2 ]]; then
        COMPREPLY=( $(compgen -W "sync cache completions" -- "$cur") )
        return
      fi
      case "${words[2]}" in
        cache)
          if [[ $cword -eq 3 ]]; then
            COMPREPLY=( $(compgen -W "status country user" -- "$cur") )
            return
          fi
          ;;
        completions)
          if [[ $cword -eq 3 ]]; then
            COMPREPLY=( $(compgen -W "bash zsh fish" -- "$cur") )
            return
          fi
          ;;
      esac
      ;;
  esac
}

complete -F _spotify_cli_completion spotify-cli
