node {
    stage('Build') {
        sh "mdbook build docs"
    }
    stage('Copy') {
        echo "Master branch received. Copying to production."
    }
}

