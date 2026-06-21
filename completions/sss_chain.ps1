# PowerShell completion for sss_chain — SSS link chain dev tool
# Usage: . ./completions/sss_chain.ps1

using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'sss_chain' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $tokens = @(
        foreach ($element in $commandAst.CommandElements) {
            if ($element -is [StringConstantExpressionAst] -and
                $element.StringConstantType -eq [StringConstantType]::BareWord) {
                $element.Value
            }
        }
    )

    $command = ($tokens -join ';')

    $completions = @(switch -Regex ($command) {
        '^sss_chain$' {
            'generate', 'validate', 'arrange', 'depth', 'demo', 'help' | ForEach-Object {
                [CompletionResult]::new($_, $_, [CompletionResultType]::ParameterValue, $_)
            }
            break
        }
        '^sss_chain;generate' {
            '--root', '--out', '--link-byte-len', '--link-count', '--total-bytes', '--quiet' | ForEach-Object {
                [CompletionResult]::new($_, $_, [CompletionResultType]::ParameterName, $_)
            }
            break
        }
        '^sss_chain;(validate|arrange|depth|demo)' {
            '--root', '--in', '--out', '--index', '--quiet' | ForEach-Object {
                [CompletionResult]::new($_, $_, [CompletionResultType]::ParameterName, $_)
            }
            break
        }
    })

    $completions | Where-Object { $_.CompletionText -like "$wordToComplete*" }
}
