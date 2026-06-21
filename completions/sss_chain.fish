# Fish completion for sss_chain — SSS link chain dev tool

complete -c sss_chain -f

complete -c sss_chain -n "__fish_use_subcommand" -a generate -d "Build chain from root"
complete -c sss_chain -n "__fish_use_subcommand" -a validate -d "Validate chain file"
complete -c sss_chain -n "__fish_use_subcommand" -a arrange -d "Reorder chain shares"
complete -c sss_chain -n "__fish_use_subcommand" -a depth -d "Query chain depth"
complete -c sss_chain -n "__fish_use_subcommand" -a demo -d "Write demo chain"
complete -c sss_chain -n "__fish_use_subcommand" -a help -d "Show usage"

complete -c sss_chain -n "__fish_seen_subcommand_from generate" -l root -r -F
complete -c sss_chain -n "__fish_seen_subcommand_from generate" -l out -r -F
complete -c sss_chain -n "__fish_seen_subcommand_from generate" -l link-byte-len -d "Bytes per link"
complete -c sss_chain -n "__fish_seen_subcommand_from generate" -l link-count -d "Number of links"
complete -c sss_chain -n "__fish_seen_subcommand_from generate" -l total-bytes -d "Total bytes"
complete -c sss_chain -n "__fish_seen_subcommand_from generate" -l quiet -d "Suppress info"

complete -c sss_chain -n "__fish_seen_subcommand_from validate arrange depth demo" -l root -r -F
complete -c sss_chain -n "__fish_seen_subcommand_from validate arrange depth demo" -l in -r -F
complete -c sss_chain -n "__fish_seen_subcommand_from validate arrange depth demo" -l out -r -F
complete -c sss_chain -n "__fish_seen_subcommand_from validate arrange depth demo" -l index -d "Link index"
complete -c sss_chain -n "__fish_seen_subcommand_from validate arrange depth demo" -l quiet -d "Suppress info"
