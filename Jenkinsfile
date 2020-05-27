node {
    stage('Build') {
        sh "$HOME/.cargo/bin/mdbook build docs"
    }
    stage('Copy') {
        echo "Master branch received. Copying to production."
    }
}

