OUTPUT = 'docs/book/'
PRODUCTION = '/var/www/docs/mdbook-latex'

node {
    stage('Build') {
        mdbook build docs
    }
    stage('Copy') {
        echo "Master branch received. Copying to production."
        sh "rsync -r ${OUTPUT} ${PRODUCTION}"
    }
}

