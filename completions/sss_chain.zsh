#compdef sss_chain

_sss_chain() {
    local line

    _arguments -C \
        '1:command:->cmds' \
        '*::args:->args'

    case $state in
        cmds)
            _values "sss_chain command" \
                'generate[Build chain from root]' \
                'validate[Validate chain file]' \
                'arrange[Reorder chain shares]' \
                'depth[Query chain depth]' \
                'demo[Write demo chain]' \
                'help[Show usage]'
            ;;
        args)
            case $line[1] in
                generate)
                    _arguments \
                        '--root[Root secret file]:file:_files' \
                        '--out[Output .ssc file]:file:_files' \
                        '--link-byte-len[Bytes per link]:count:' \
                        '--link-count[Number of links]:count:' \
                        '--total-bytes[Total bytes]:count:' \
                        '--quiet[Suppress info]'
                    ;;
                validate|arrange|depth|demo)
                    _files
                    ;;
            esac
            ;;
    esac
}

_sss_chain "$@"
