package main

import (
	"os"

	"git.front.kjuulh.io/kjuulh/octopush/cmd/server/commands"
	"git.front.kjuulh.io/kjuulh/octopush/internal/logger"
	"go.uber.org/zap"
)

func main() {
	logger, err := logger.New()
	if err != nil {
		panic(err)
	}
	_ = logger.Sync()

	zap.ReplaceGlobals(logger)

	Execute(logger)
}

func Execute(logger *zap.Logger) {
	err := commands.CreateServerCmd(logger).Execute()
	if err != nil {
		logger.Error("execution failed", zap.Error(err))
		os.Exit(1)
	}
}
