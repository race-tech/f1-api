USE f1db;

CREATE TABLE `circuits` (
  `circuitId` int(11) NOT NULL AUTO_INCREMENT,
  `circuitRef` varchar(255) NOT NULL DEFAULT '',
  `name` varchar(255) NOT NULL DEFAULT '',
  `location` varchar(255) DEFAULT NULL,
  `country` varchar(255) DEFAULT NULL,
  `lat` float DEFAULT NULL,
  `lng` float DEFAULT NULL,
  `alt` int(11) DEFAULT NULL,
  `url` varchar(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`circuitId`),
  UNIQUE KEY `url` (`url`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8;

CREATE TABLE `constructorResults` (
  `constructorResultsId` int(11) NOT NULL AUTO_INCREMENT,
  `raceId` int(11) NOT NULL DEFAULT '0',
  `constructorId` int(11) NOT NULL DEFAULT '0',
  `points` float DEFAULT NULL,
  `status` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`constructorResultsId`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8;

CREATE TABLE `constructorStandings` (
  `constructorStandingsId` int(11) NOT NULL AUTO_INCREMENT,
  `raceId` int(11) NOT NULL DEFAULT '0',
  `constructorId` int(11) NOT NULL DEFAULT '0',
  `points` float NOT NULL DEFAULT '0',
  `position` int(11) DEFAULT NULL,
  `positionText` varchar(255) DEFAULT NULL,
  `wins` int(11) NOT NULL DEFAULT '0',
  PRIMARY KEY (`constructorStandingsId`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8;

CREATE TABLE `constructors` (
  `constructorId` int(11) NOT NULL AUTO_INCREMENT,
  `constructorRef` varchar(255) NOT NULL DEFAULT '',
  `name` varchar(255) NOT NULL DEFAULT '',
  `nationality` varchar(255) DEFAULT NULL,
  `url` varchar(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`constructorId`),
  UNIQUE KEY `name` (`name`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8;

CREATE TABLE `driverStandings` (
  `driverStandingsId` int(11) NOT NULL AUTO_INCREMENT,
  `raceId` int(11) NOT NULL DEFAULT '0',
  `driverId` int(11) NOT NULL DEFAULT '0',
  `points` float NOT NULL DEFAULT '0',
  `position` int(11) DEFAULT NULL,
  `positionText` varchar(255) DEFAULT NULL,
  `wins` int(11) NOT NULL DEFAULT '0',
  PRIMARY KEY (`driverStandingsId`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8;

CREATE TABLE `drivers` (
  `driverId` int(11) NOT NULL AUTO_INCREMENT,
  `driverRef` varchar(255) NOT NULL DEFAULT '',
  `number` int(11) DEFAULT NULL,
  `code` varchar(3) DEFAULT NULL,
  `forename` varchar(255) NOT NULL DEFAULT '',
  `surname` varchar(255) NOT NULL DEFAULT '',
  `dob` date DEFAULT NULL,
  `nationality` varchar(255) DEFAULT NULL,
  `url` varchar(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`driverId`),
  UNIQUE KEY `url` (`url`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8;

CREATE TABLE `lapTimes` (
  `raceId` int(11) NOT NULL,
  `driverId` int(11) NOT NULL,
  `lap` int(11) NOT NULL,
  `position` int(11) DEFAULT NULL,
  `time` varchar(255) DEFAULT NULL,
  `milliseconds` int(11) DEFAULT NULL,
  PRIMARY KEY (`raceId`,`driverId`,`lap`),
  KEY `raceId` (`raceId`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8;

CREATE TABLE `pitStops` (
  `raceId` int(11) NOT NULL,
  `driverId` int(11) NOT NULL,
  `stop` int(11) NOT NULL,
  `lap` int(11) NOT NULL,
  `time` time NOT NULL,
  `duration` varchar(255) DEFAULT NULL,
  `milliseconds` int(11) DEFAULT NULL,
  PRIMARY KEY (`raceId`,`driverId`,`stop`),
  KEY `raceId` (`raceId`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8;

CREATE TABLE `qualifying` (
  `qualifyId` int(11) NOT NULL AUTO_INCREMENT,
  `raceId` int(11) NOT NULL DEFAULT '0',
  `driverId` int(11) NOT NULL DEFAULT '0',
  `constructorId` int(11) NOT NULL DEFAULT '0',
  `number` int(11) NOT NULL DEFAULT '0',
  `position` int(11) DEFAULT NULL,
  `q1` varchar(255) DEFAULT NULL,
  `q2` varchar(255) DEFAULT NULL,
  `q3` varchar(255) DEFAULT NULL,
  PRIMARY KEY (`qualifyId`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8;

CREATE TABLE `races` (
  `raceId` int(11) NOT NULL AUTO_INCREMENT,
  `year` int(11) NOT NULL DEFAULT '0',
  `round` int(11) NOT NULL DEFAULT '0',
  `circuitId` int(11) NOT NULL DEFAULT '0',
  `name` varchar(255) NOT NULL DEFAULT '',
  `date` date NOT NULL DEFAULT '0000-00-00',
  `time` time DEFAULT NULL,
  `url` varchar(255) DEFAULT NULL,
  `fp1_date` date DEFAULT NULL,
  `fp1_time` time DEFAULT NULL,
  `fp2_date` date DEFAULT NULL,
  `fp2_time` time DEFAULT NULL,
  `fp3_date` date DEFAULT NULL,
  `fp3_time` time DEFAULT NULL,
  `quali_date` date DEFAULT NULL,
  `quali_time` time DEFAULT NULL,
  `sprint_date` date DEFAULT NULL,
  `sprint_time` time DEFAULT NULL,
  PRIMARY KEY (`raceId`),
  UNIQUE KEY `url` (`url`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8;

CREATE TABLE `results` (
  `resultId` int(11) NOT NULL AUTO_INCREMENT,
  `raceId` int(11) NOT NULL DEFAULT '0',
  `driverId` int(11) NOT NULL DEFAULT '0',
  `constructorId` int(11) NOT NULL DEFAULT '0',
  `number` int(11) cscNOT NULL DEFAULT '0',
  `grid` int(11) NOT NULL DEFAULT '0',
  `position` int(11) DEFAULT NULL,
  `positionText` varchar(255) NOT NULL DEFAULT '',
  `positionOrder` int(11) NOT NULL DEFAULT '0',
  `points` float NOT NULL DEFAULT '0',
  `laps` int(11) NOT NULL DEFAULT '0',
  `time` varchar(255) DEFAULT NULL,
  `milliseconds` int(11) DEFAULT NULL,
  `fastestLap` int(11) DEFAULT NULL,
  `rank` int(11) DEFAULT '0',
  `fastestLapTime` varchar(255) DEFAULT NULL,
  `fastestLapSpeed` varchar(255) DEFAULT NULL,
  `statusId` int(11) NOT NULL DEFAULT '0',
  PRIMARY KEY (`resultId`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8;

CREATE TABLE `seasons` (
  `year` int(11) NOT NULL DEFAULT '0',
  `url` varchar(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`year`),
  UNIQUE KEY `url` (`url`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8;


CREATE TABLE `sprintResults` (
  `sprintResultId` int(11) NOT NULL AUTO_INCREMENT,
  `raceId` int(11) NOT NULL DEFAULT '0',
  `driverId` int(11) NOT NULL DEFAULT '0',
  `constructorId` int(11) NOT NULL DEFAULT '0',
  `number` int(11) NOT NULL DEFAULT '0',
  `grid` int(11) NOT NULL DEFAULT '0',
  `position` int(11) DEFAULT NULL,
  `positionText` varchar(255) NOT NULL DEFAULT '',
  `positionOrder` int(11) NOT NULL DEFAULT '0',
  `points` float NOT NULL DEFAULT '0',
  `laps` int(11) NOT NULL DEFAULT '0',
  `time` varchar(255) DEFAULT NULL,
  `milliseconds` int(11) DEFAULT NULL,
  `fastestLap` int(11) DEFAULT NULL,
  `fastestLapTime` varchar(255) DEFAULT NULL,
  `statusId` int(11) NOT NULL DEFAULT '0',
  PRIMARY KEY (`sprintResultId`),
  KEY `raceId` (`raceId`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8;


CREATE TABLE `status` (
  `statusId` int(11) NOT NULL AUTO_INCREMENT,
  `status` varchar(255) NOT NULL DEFAULT '',
  PRIMARY KEY (`statusId`)
) ENGINE=MyISAM DEFAULT CHARSET=utf8;
