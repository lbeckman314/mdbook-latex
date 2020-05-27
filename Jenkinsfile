OUTPUT = 'docs/book/'
PRODUCTION = '/var/www/docs/mdbook-latex'

node {
    stage('Update') {
        git url: 'https://github.com/lbeckman314/mdbook-latex/'
    }
    stage('Build') {
        sh "$HOME/.cargo/bin/mdbook build docs"
    }
    stage('Copy') {
        sh "rsync -r ${OUTPUT} ${PRODUCTION}"
    }
}

