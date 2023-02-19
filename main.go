package main

import (
	"fmt"
	_ "github.com/rajatsharma/giganigma/tiny-proc"
	"log"
	"os"
	"os/exec"
	"path"
)

func exists(path string) bool {
	_, err := os.Stat(path)
	if err == nil {
		return true
	}
	if os.IsNotExist(err) {
		return false
	}

	panic(fmt.Sprintf("error while reading %s occured: %v", path, err))
}

func proc(shell string) {
	cwd, _ := os.Getwd()
	log.Printf("Running %s in %s", shell, cwd)

	out, err := exec.Command("sh", "-c", shell).CombinedOutput()
	fmt.Println(string(out))

	if err != nil {
		log.Panicf("Unable to run %s", shell)
	}
}

func cleanup(project string) {
	log.Println("Cleaning up")
	if err := os.RemoveAll(path.Join(project)); err != nil {
		log.Fatalf("Could not delete project dir, remove yourself %s", project)
	}
}

func main() {
	repo := os.Args[1]

	if repo == "" {
		panic("No args passed. Expected repo_name")
	}

	previousDir, err := os.Getwd()
	if err != nil {
		log.Fatalln("Could not get current dir")
	}

	home, err := os.UserHomeDir()
	if err != nil {
		log.Fatalln("Could not get home dir")
	}

	pnpmm := path.Join(home, ".pnpmm-go")

	if err = os.MkdirAll(pnpmm, os.ModePerm); err != nil {
		log.Fatalf("Unable to create dir %s", pnpmm)
	}

	if err = os.Chdir(pnpmm); err != nil {
		log.Fatalf("Could not change dir to %s", pnpmm)
	}

	proc(fmt.Sprintf("git clone https://github.com/rajatsharma/%s", repo))

	project := path.Join(home, ".pnpmm-go", repo)

	defer cleanup(project)

	if exists(path.Join(project, "pnpm-lock.yaml")) {
		log.Panicf("Already a pnpm project")
	}

	if err := os.Chdir(project); err != nil {
		log.Panicf("Could not change dir to %s", project)
	}

	proc("pnpm import")

	if err := os.Remove(path.Join(project, "yarn.lock")); err != nil {
		log.Panicln("Unable to delete yarn.lock")
	}

	proc("git add .")
	proc("git commit -m 'move to pnpm'")
	proc("git push origin master")

	if err := os.Chdir(previousDir); err != nil {
		log.Panicf("Could not change dir to %s", project)
	}
}
