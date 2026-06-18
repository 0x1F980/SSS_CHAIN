# Bash completion for sss_chain CLI

_sss_chain_completions() {
    local cur prev opts
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    opts="generate validate arrange depth demo help -h --help"

    case "$prev" in
        --root|--in|--out)
            COMPREPLY=( $(compgen -f -- "$cur") $(compgen -W "-" -- "$cur") )
            return 0
            ;;
        generate)
            COMPREPLY=( $(compgen -W "--root --link-byte-len --link-count --total-bytes --out --quiet" -- "$cur") )
            return 0
            ;;
        validate)
            COMPREPLY=( $(compgen -W "--root --in --quiet" -- "$cur") )
            return 0
            ;;
        arrange)
            COMPREPLY=( $(compgen -W "--root --in --out --quiet" -- "$cur") )
            return 0
            ;;
        depth)
            COMPREPLY=( $(compgen -W "--root --in --index --quiet" -- "$cur") )
            return 0
            ;;
        demo)
            COMPREPLY=( $(compgen -W "--out --quiet" -- "$cur") )
            return 0
            ;;
        *)
            ;;
    esac

    COMPREPLY=( $(compgen -W "$opts" -- "$cur") )
    return 0
}

complete -F _sss_chain_completions sss_chain
