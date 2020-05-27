OUTPUT = 'docs/book/'
PRODUCTION = '/var/www/docs/mdbook-latex'

node {
    stage('Build') {
        mdbook build docs
    }
    if (env.BRANCH_NAME == 'master') {
        stage('Copy') {
            echo "Master branch received. Copying to production."
            sh "rsync -r ${OUTPUT} ${PRODUCTION}"
        }
    } else {
        echo "Non-master branch received. Not copying to production."
    }
}

